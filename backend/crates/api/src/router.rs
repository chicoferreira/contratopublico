use axum::{
    Router,
    extract::{Path, State},
    middleware,
    routing::{get, post},
};
use common::Contract;
use serde::Deserialize;
use tracing::debug;

use crate::{
    error::AppError,
    extractors::Json,
    filter::Filters,
    metrics,
    sort::SortBy,
    state::{AppState, SearchResponse},
    statistics::Statistics,
};

pub fn router(app_state: AppState) -> Router {
    Router::new()
        .route("/api/search", post(search))
        .route("/api/statistics", get(statistics))
        .route("/api/contract/{id}", get(contract))
        .route_layer(middleware::from_fn(metrics::track_metrics_layer))
        .with_state(app_state)
}

#[tracing::instrument(skip(state))]
#[axum::debug_handler]
pub async fn statistics(State(state): State<AppState>) -> Result<Json<Statistics>, AppError> {
    Ok(Json(state.get_statistics()))
}

#[derive(Debug, Deserialize)]
pub struct SearchQuery {
    pub query: String,
    pub filters: Option<Filters>,
    pub sort: Option<SortBy>,
    pub page: Option<usize>,
}

#[tracing::instrument(skip(state))]
#[axum::debug_handler]
// TODO: add rate limiting
pub async fn search(
    State(state): State<AppState>,
    Json(query): Json<SearchQuery>,
) -> Result<Json<SearchResponse>, AppError> {
    // TODO: add maximum query length
    let sort = query.sort.unwrap_or_default();
    let sort = sort.to_meilisearch();

    let page = query.page.unwrap_or(1);
    let filters = query.filters.as_ref();

    // TODO: make this configurable
    const HITS_PER_PAGE: usize = 20;

    let response = state
        .search(&query.query, filters, &sort, page, HITS_PER_PAGE)
        .await?;

    debug!("Returning {} results", response.contracts.len());

    Ok(Json(response))
}

#[tracing::instrument(skip(state))]
#[axum::debug_handler]
pub async fn contract(
    State(state): State<AppState>,
    Path(id): Path<u64>,
) -> Result<Json<Option<Contract>>, AppError> {
    let contract = state.get_contract(id).await?;

    debug!("Contract with ID {} retrieved", id);

    Ok(Json(contract))
}
