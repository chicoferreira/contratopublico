use std::{
    convert::Infallible,
    net::{IpAddr, SocketAddr},
};

use axum::{
    extract::{
        ConnectInfo, FromRequest, FromRequestParts, OptionalFromRequestParts, Request,
        rejection::JsonRejection,
    },
    http::request::Parts,
    response::IntoResponse,
};
use garde::{Unvalidated, Valid, Validate};
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

pub struct ValidJson<T>(pub Valid<T>);

impl<S, T> FromRequest<S> for ValidJson<T>
where
    Json<T>: FromRequest<S, Rejection = AppError>,
    T: Validate,
    T::Context: Default,
    S: Send + Sync,
{
    type Rejection = AppError;

    async fn from_request(request: Request, state: &S) -> Result<Self, Self::Rejection> {
        let Json(value) = Json::<T>::from_request(request, state).await?;
        Unvalidated::new(value)
            .validate()
            .map(ValidJson)
            .map_err(|report| AppError::InvalidRequest(report.to_string().trim_end().to_owned()))
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

const CF_IP_COUNTRY_HEADER: &str = "CF-IPCountry";

pub struct ClientCountry(pub String);

impl<S> OptionalFromRequestParts<S> for ClientCountry
where
    S: Send + Sync,
{
    type Rejection = Infallible;

    async fn from_request_parts(
        parts: &mut Parts,
        _state: &S,
    ) -> Result<Option<Self>, Self::Rejection> {
        Ok(parts
            .headers
            .get(CF_IP_COUNTRY_HEADER)
            .and_then(|v| v.to_str().ok())
            .map(|country| ClientCountry(country.to_owned())))
    }
}
