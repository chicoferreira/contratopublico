use std::net::{IpAddr, SocketAddr};

use axum::{
    extract::{ConnectInfo, FromRequest, FromRequestParts, rejection::JsonRejection},
    http::request::Parts,
    response::IntoResponse,
};
use serde::Serialize;

use crate::error::AppError;

#[derive(FromRequest)]
#[from_request(via(axum::Json), rejection(AppError))]
pub struct Json<T>(pub T);

impl From<JsonRejection> for AppError {
    fn from(rejection: JsonRejection) -> Self {
        Self::JsonParseError(rejection.body_text())
    }
}

impl<T: Serialize> IntoResponse for Json<T> {
    fn into_response(self) -> axum::response::Response {
        let Self(value) = self;
        axum::Json(value).into_response()
    }
}

const CF_CONNECTING_IP_HEADER: &str = "CF-Connecting-IP";

pub struct ClientIp(pub IpAddr);

impl<S> FromRequestParts<S> for ClientIp
where
    S: Send + Sync,
{
    type Rejection = AppError;

    async fn from_request_parts(parts: &mut Parts, state: &S) -> Result<Self, Self::Rejection> {
        let cf_ip = parts
            .headers
            .get(CF_CONNECTING_IP_HEADER)
            .and_then(|v| v.to_str().ok())
            .and_then(|s| s.parse::<IpAddr>().ok())
            .map(ClientIp);

        match cf_ip {
            Some(ip) => Ok(ip),
            None => ConnectInfo::<SocketAddr>::from_request_parts(parts, state)
                .await
                .map(|ConnectInfo(addr)| ClientIp(addr.ip()))
                .map_err(|_| AppError::MissingClientIp),
        }
    }
}
