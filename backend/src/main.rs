use axum::{
    Json, Router,
    extract::{Query, State},
    http::{Response, StatusCode},
    response::IntoResponse,
    routing::get,
};
use common::Contract;
use log;
use meilisearch_sdk::client::Client;
use serde::{Deserialize, Serialize};

use crate::state::{AppError, AppState};

mod state;

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();

    let meilisearch = Client::new(
        "http://localhost:7700",
        Some("MASTER_KEY_TO_CHANGE_IN_PRODUCTION"),
    )
    .expect("Failed to create Meilisearch client");

    let app_state = AppState::new(meilisearch);

    let app = Router::new().route("/search", get(search));

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app.with_state(app_state))
        .await
        .unwrap();
}

#[derive(Debug, Deserialize)]
struct SearchQuery {
    query: String,
    page: Option<usize>,
    size: Option<usize>,
}

#[derive(Debug, Serialize)]
struct SearchResponse(Vec<Contract>);

#[axum::debug_handler]
async fn search(
    Query(query): Query<SearchQuery>,
    State(state): State<AppState>,
) -> Result<Json<SearchResponse>, AppError> {
    let results = state.search(&query.query).await?;
    // TODO: implement pagination and return formatted results
    Ok(Json(SearchResponse(
        results.hits.into_iter().map(|hit| hit.result).collect(),
    )))
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response<axum::body::Body> {
        match self {
            AppError::MeilisearchError(e) => {
                log::error!("Meilisearch error: {}", e);
                StatusCode::INTERNAL_SERVER_ERROR.into_response()
            }
        }
    }
}
