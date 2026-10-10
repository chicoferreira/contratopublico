use std::{
    sync::atomic::{AtomicUsize, Ordering},
    time::Duration,
};

use governor::{DefaultDirectRateLimiter, Quota, RateLimiter};
use tokio::sync::Semaphore;

use crate::base_gov::{
    BaseGovContract, ContractSearchResponse,
    client::{BaseGovClient, ContractSort},
};

pub const MAX_CONSECUTIVE_FAILURES: usize = 5;
const RETRY_DELAY: Duration = Duration::from_secs(15);

/// A [`BaseGovClient`] that rate limits requests and backs off exponentially after failures.
pub struct ThrottledClient {
    client: BaseGovClient,
    rate_limiter: DefaultDirectRateLimiter,
    semaphore: Semaphore,
    consecutive_failures: AtomicUsize,
}

impl ThrottledClient {
    /// Allows at most `max_concurrent` requests at once, spaced by `rate_limit_quota`.
    pub fn new(client: BaseGovClient, max_concurrent: usize, rate_limit_quota: Quota) -> Self {
        Self {
            client,
            rate_limiter: RateLimiter::direct(rate_limit_quota),
            semaphore: Semaphore::new(max_concurrent),
            consecutive_failures: AtomicUsize::new(0),
        }
    }

    /// Fetches a page of contract IDs. Returns `None` once the client has given up.
    pub async fn fetch_page(
        &self,
        sort: ContractSort,
        page: usize,
        size: usize,
    ) -> Option<anyhow::Result<ContractSearchResponse>> {
        self.request(self.client.fetch_page(sort, page, size)).await
    }

    /// Fetches a contract's details. Returns `None` once the client has given up.
    pub async fn get_contract_details(&self, id: u64) -> Option<anyhow::Result<BaseGovContract>> {
        self.request(self.client.get_contract_details(id)).await
    }

    /// Runs `request` after waiting for a free slot, the backoff and the rate limit.
    ///
    /// Returns `None` without running it after `MAX_CONSECUTIVE_FAILURES` failures in a row.
    async fn request<T>(
        &self,
        request: impl Future<Output = anyhow::Result<T>>,
    ) -> Option<anyhow::Result<T>> {
        let _permit = self.semaphore.acquire().await.unwrap();
        let failures = self.consecutive_failures.load(Ordering::Relaxed);
        if failures >= MAX_CONSECUTIVE_FAILURES {
            return None;
        }
        if failures > 0 {
            tokio::time::sleep(RETRY_DELAY * 2_u32.pow(failures as u32 - 1)).await;
        }
        self.rate_limiter.until_ready().await;

        let response = request.await;
        if response.is_ok() {
            self.consecutive_failures.store(0, Ordering::Relaxed);
        } else {
            self.consecutive_failures.fetch_add(1, Ordering::Relaxed);
        }
        Some(response)
    }
}
