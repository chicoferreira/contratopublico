use axum::{
    extract::Request,
    http::{Method, Uri},
    middleware::Next,
    response::Response,
};
use axum_extra::{TypedHeader, headers::UserAgent, typed_header::TypedHeaderRejection};
use tokio::time::Instant;
use tracing::info;

use crate::{
    error::AppError,
    extractors::{ClientCountry, ClientIp},
};

#[derive(Clone)]
pub struct ErrorField(pub String);

pub async fn access_log_layer(
    ip: Result<ClientIp, AppError>,
    user_agent: Result<TypedHeader<UserAgent>, TypedHeaderRejection>,
    country: Option<ClientCountry>,
    method: Method,
    uri: Uri,
    request: Request,
    next: Next,
) -> Response {
    let start = Instant::now();

    let response = next.run(request).await;

    info!(
        method = %method,
        path = uri.path(),
        status = response.status().as_u16(),
        latency_ms = start.elapsed().as_micros() as f64 / 1000.0,
        ip = ip.ok().map(|ClientIp(ip)| display(ip)),
        user_agent = user_agent.ok().map(|TypedHeader(user_agent)| display(user_agent)),
        country = country.map(|ClientCountry(country)| country),
        error = response.extensions().get::<ErrorField>().map(|ErrorField(error)| error.as_str()),
        "request"
    );

    response
}
