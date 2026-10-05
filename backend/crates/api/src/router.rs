use std::{num::NonZero, sync::Arc, time::Duration};

use axum::{
    Router,
    extract::{Path, State},
    middleware,
    routing::{get, post},
};
use common::{Contract, statistics::Statistics};
use governor::Quota;
use serde::Deserialize;

use crate::{
    access_log,
    blocklist::{Blocklist, blocklist_layer},
    error::AppError,
    extractors::Json,
    filter::Filters,
    metrics,
    rate_limit::RateLimitLayer,
    sort::SortBy,
    state::{AppState, SearchResponse},
};

pub fn router(app_state: AppState, blocklist: Arc<Blocklist>) -> Router {
    let contract_rate_limit = Quota::with_period(Duration::from_millis(200))
        .unwrap()
        .allow_burst(NonZero::try_from(2).unwrap());

    Router::new()
        .merge(
            Router::new()
                .route("/api/search", post(search))
                .route("/api/contract/{id}", get(contract))
                .route_layer(RateLimitLayer::new(contract_rate_limit)),
        )
        .route("/api/statistics", get(statistics))
        .route_layer(middleware::from_fn_with_state(blocklist, blocklist_layer))
        .route_layer(middleware::from_fn(metrics::track_metrics_layer))
        .layer(middleware::from_fn(access_log::access_log_layer))
        .with_state(app_state)
}

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

#[axum::debug_handler]
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

    Ok(Json(response))
}

#[axum::debug_handler]
pub async fn contract(
    State(state): State<AppState>,
    Path(id): Path<u64>,
) -> Result<Json<Option<Contract>>, AppError> {
    let contract = state.get_contract(id).await?;

    Ok(Json(contract))
}
