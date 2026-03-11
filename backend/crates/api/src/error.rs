use axum::{http::Response, response::IntoResponse};
use reqwest::StatusCode;
use serde::Serialize;
use tracing::error;

#[derive(thiserror::Error, Debug)]
pub enum AppError {
    #[error(transparent)]
    MeilisearchError(#[from] meilisearch_sdk::errors::Error),
    #[error("Invalid JSON: {0}")]
    JsonParseError(String),
    #[error(transparent)]
    DatabaseError(#[from] sqlx::Error),
    #[error("Could not determine client IP address")]
    MissingClientIp,
    #[error("Too many requests")]
    RateLimited,
}

pub type AppResult<T> = Result<T, AppError>;

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
            AppError::JsonParseError(message) => (
                StatusCode::BAD_REQUEST,
                format!("Invalid JSON: {}", message),
            ),
            AppError::DatabaseError(e) => {
                error!("Database error: {:?}", e);
                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    format!("A failure from Database has occurred"),
                )
            }
            AppError::MissingClientIp => (
                StatusCode::INTERNAL_SERVER_ERROR,
                format!("Could not determine client IP address"),
            ),
            AppError::RateLimited => (
                StatusCode::TOO_MANY_REQUESTS,
                format!("Too many requests"),
            ),
        };
        let error_body = ErrorBody { message };
        (error_code, axum::Json(error_body)).into_response()
    }
}
