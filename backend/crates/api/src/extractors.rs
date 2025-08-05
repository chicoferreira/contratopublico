use crate::state::AppError;
use axum::{
    extract::{FromRequest, rejection::JsonRejection},
    http::StatusCode,
    response::{IntoResponse, Response},
};
use serde::Serialize;
use tracing::error;

#[derive(Debug, Serialize)]
struct ErrorBody {
    message: String,
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response<axum::body::Body> {
        let (error_code, message) = match self {
            AppError::MeilisearchError(e) => {
                error!("Meilisearch error: {:?}", e);
                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    format!("A failure from Meilisearch has occurred"),
                )
            }
            AppError::JsonParseError(message) => {
                error!("JSON parse error: {}", message);
                (
                    StatusCode::BAD_REQUEST,
                    format!("Invalid JSON: {}", message),
                )
            }
        };
        let error_body = ErrorBody { message };
        (error_code, axum::Json(error_body)).into_response()
    }
}

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
