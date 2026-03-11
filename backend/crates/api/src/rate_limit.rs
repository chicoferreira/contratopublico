use std::{
    future::Future,
    net::IpAddr,
    pin::Pin,
    sync::Arc,
    task::{Context, Poll},
    time::{Duration, Instant},
};

use axum::{
    RequestExt,
    body::Body,
    extract::Request,
    response::{IntoResponse, Response},
};
use dashmap::DashMap;
use governor::{DefaultKeyedRateLimiter, Quota, RateLimiter};
use tower::{Layer, Service};
use tracing::warn;

use crate::{error::AppError, extractors::ClientIp};

const ENTRIES_FOR_GC: usize = 100;
const BASE_PENALTY_DURATION: Duration = Duration::from_secs(5);
const MAX_PENALTY_DURATION: Duration = Duration::from_secs(60);
const ENTRY_MAP_TTL: Duration = Duration::from_secs(120);

#[derive(Debug)]
struct PenaltyEntry {
    until: Instant,
    map_expires_at: Instant,
    violations: u32,
}

impl PenaltyEntry {
    fn is_penalized(&self) -> bool {
        Instant::now() < self.until
    }

    fn is_expired(&self) -> bool {
        Instant::now() >= self.map_expires_at
    }
}

struct PenaltyMap {
    inner: DashMap<IpAddr, PenaltyEntry>,
}

impl PenaltyMap {
    fn new() -> Self {
        Self {
            inner: DashMap::new(),
        }
    }

    fn check(&self, ip: &IpAddr) -> bool {
        self.inner.get(ip).is_some_and(|entry| entry.is_penalized())
    }

    fn penalize(&self, ip: IpAddr) {
        if self.inner.len() >= ENTRIES_FOR_GC {
            self.inner.retain(|_, entry| !entry.is_expired());
        }

        let violations = self
            .inner
            .get(&ip)
            .filter(|e| !e.is_expired())
            .map_or(0, |e| e.violations)
            + 1;

        let duration = BASE_PENALTY_DURATION
            .saturating_mul(2u32.saturating_pow(violations - 1))
            .min(MAX_PENALTY_DURATION);

        let now = Instant::now();

        warn!(
            ip = %ip,
            violations,
            duration_secs = duration.as_secs(),
            "IP penalized for repeated rate-limit violations"
        );

        self.inner.insert(
            ip,
            PenaltyEntry {
                until: now + duration,
                map_expires_at: now + ENTRY_MAP_TTL,
                violations,
            },
        );
    }
}

#[derive(Clone)]
pub struct RateLimitLayer {
    limiter: Arc<DefaultKeyedRateLimiter<IpAddr>>,
    penalties: Arc<PenaltyMap>,
}

impl RateLimitLayer {
    pub fn new(quota: Quota) -> Self {
        Self {
            limiter: Arc::new(RateLimiter::keyed(quota)),
            penalties: Arc::new(PenaltyMap::new()),
        }
    }
}

impl<S> Layer<S> for RateLimitLayer {
    type Service = RateLimitService<S>;

    fn layer(&self, inner: S) -> Self::Service {
        RateLimitService {
            inner,
            limiter: self.limiter.clone(),
            penalties: self.penalties.clone(),
        }
    }
}

#[derive(Clone)]
pub struct RateLimitService<S> {
    inner: S,
    limiter: Arc<DefaultKeyedRateLimiter<IpAddr>>,
    penalties: Arc<PenaltyMap>,
}

impl<S> Service<Request<Body>> for RateLimitService<S>
where
    S: Service<Request<Body>, Response = Response> + Clone + Send + 'static,
    S::Future: Send + 'static,
    S::Error: Send + 'static,
{
    type Response = Response;
    type Error = S::Error;
    type Future = Pin<Box<dyn Future<Output = Result<Response, S::Error>> + Send>>;

    fn poll_ready(&mut self, cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        self.inner.poll_ready(cx)
    }

    fn call(&mut self, mut request: Request<Body>) -> Self::Future {
        let mut inner = self.inner.clone();
        let limiter = self.limiter.clone();
        let penalties = self.penalties.clone();

        Box::pin(async move {
            let ClientIp(ip) = match request.extract_parts::<ClientIp>().await {
                Ok(ip) => ip,
                Err(e) => return Ok(e.into_response()),
            };

            if penalties.check(&ip) {
                return Ok(AppError::RateLimited.into_response());
            }

            if limiter.check_key(&ip).is_err() {
                penalties.penalize(ip);
                return Ok(AppError::RateLimited.into_response());
            }

            inner.call(request).await
        })
    }
}
