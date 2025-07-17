use anyhow::Context;
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
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt::init();

    let meilisearch = Client::new(
        "http://localhost:7700",
        Some("MASTER_KEY_TO_CHANGE_IN_PRODUCTION"),
    )
    .expect("Failed to create Meilisearch client");

    let app_state = AppState::new(meilisearch);
    app_state.prepare_indexes().await?;

    let app = Router::new().route("/search", get(search));

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app.with_state(app_state))
        .await
        .context("Failed to serve axum")
}

#[derive(Debug, Deserialize)]
struct SearchQuery {
    query: String,
    filter: Option<String>,
    sort: Option<Vec<String>>,
    page: Option<usize>,
    offset: Option<usize>,
}

#[derive(Debug, Serialize)]
struct SearchResponse {
    contracts: Vec<Contract>,
    total: Option<usize>,
    estimated_total: Option<usize>,
    page: usize,
    offset: usize,
}

#[axum::debug_handler]
async fn search(
    Query(query): Query<SearchQuery>,
    State(state): State<AppState>,
) -> Result<Json<SearchResponse>, AppError> {
    let sort = query.sort.unwrap_or_default();
    let sort: Vec<&str> = sort.iter().map(|x| &**x).collect();

    let filter = query.filter.unwrap_or_default();

    let page = query.page.unwrap_or(1);
    let offset = query.offset.unwrap_or(0);

    const HITS_PER_PAGE: usize = 50;

    let results = state
        .search(&query.query, &filter, &sort, page, offset, HITS_PER_PAGE)
        .await?;

    // TODO: return formatted results
    Ok(Json(SearchResponse {
        contracts: results.hits.into_iter().map(|hit| hit.result).collect(),
        page: results.page.unwrap_or(0),
        offset: results.offset.unwrap_or(0),
        total: results.total_hits,
        estimated_total: results.estimated_total_hits,
    }))
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
