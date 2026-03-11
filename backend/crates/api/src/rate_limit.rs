use std::{
    future::Future,
    net::IpAddr,
    pin::Pin,
    sync::Arc,
    task::{Context, Poll},
};

use axum::{
    RequestExt,
    body::Body,
    extract::Request,
    response::{IntoResponse, Response},
};
use governor::{DefaultKeyedRateLimiter, Quota, RateLimiter};
use tower::{Layer, Service};

use crate::{error::AppError, extractors::ClientIp};

#[derive(Clone)]
pub struct RateLimitLayer {
    limiter: Arc<DefaultKeyedRateLimiter<IpAddr>>,
}

impl RateLimitLayer {
    pub fn new(quota: Quota) -> Self {
        Self {
            limiter: Arc::new(RateLimiter::keyed(quota)),
        }
    }
}

impl<S> Layer<S> for RateLimitLayer {
    type Service = RateLimitService<S>;

    fn layer(&self, inner: S) -> Self::Service {
        RateLimitService {
            inner,
            limiter: self.limiter.clone(),
        }
    }
}

#[derive(Clone)]
pub struct RateLimitService<S> {
    inner: S,
    limiter: Arc<DefaultKeyedRateLimiter<IpAddr>>,
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

        Box::pin(async move {
            let ClientIp(ip) = match request.extract_parts::<ClientIp>().await {
                Ok(ip) => ip,
                Err(e) => return Ok(e.into_response()),
            };

            if limiter.check_key(&ip).is_err() {
                return Ok(AppError::RateLimited.into_response());
            }

            inner.call(request).await
        })
    }
}
