use std::{
    sync::atomic::{AtomicU32, Ordering},
    time::Duration,
};

use governor::{DefaultDirectRateLimiter, Quota, RateLimiter};

use crate::sources::base_portal::{
    BasePortalContract,
    client::{BasePortalClient, ContractSearchResponse, ContractSort},
};

const RETRY_DELAY: Duration = Duration::from_secs(15);
const MAX_RETRY_DELAY: Duration = Duration::from_hours(1);

/// A [`BasePortalClient`] that rate limits requests and backs off exponentially after failures.
pub struct ThrottledClient {
    client: BasePortalClient,
    rate_limiter: DefaultDirectRateLimiter,
    consecutive_failures: AtomicU32,
}

impl ThrottledClient {
    /// Spaces requests by `rate_limit_quota`.
    pub fn new(client: BasePortalClient, rate_limit_quota: Quota) -> Self {
        Self {
            client,
            rate_limiter: RateLimiter::direct(rate_limit_quota),
            consecutive_failures: AtomicU32::new(0),
        }
    }

    pub async fn fetch_page(
        &self,
        sort: ContractSort,
        page: usize,
        size: usize,
    ) -> anyhow::Result<ContractSearchResponse> {
        self.request(self.client.fetch_page(sort, page, size)).await
    }

    pub async fn get_contract_details(
        &self,
        id: u64,
    ) -> anyhow::Result<Option<BasePortalContract>> {
        self.request(self.client.get_contract_details(id)).await
    }

    /// Runs `request` after waiting for the backoff and the rate limit.
    ///
    /// The backoff doubles after each consecutive failure, up to `MAX_RETRY_DELAY`.
    async fn request<T>(
        &self,
        request: impl Future<Output = anyhow::Result<T>>,
    ) -> anyhow::Result<T> {
        let failures = self.consecutive_failures.load(Ordering::Relaxed);
        if failures > 0 {
            let delay = RETRY_DELAY.saturating_mul(2_u32.saturating_pow(failures - 1));
            tokio::time::sleep(delay.min(MAX_RETRY_DELAY)).await;
        }
        self.rate_limiter.until_ready().await;

        let response = request.await;
        if response.is_ok() {
            self.consecutive_failures.store(0, Ordering::Relaxed);
        } else {
            self.consecutive_failures.fetch_add(1, Ordering::Relaxed);
        }
        response
    }
}
