use std::collections::HashMap;

use axum::extract::{Json, State};
use common::Contract;
use serde::{Deserialize, Serialize};
use tracing::debug;

use crate::{
    sort::SortBy,
    state::{AppError, AppState},
};

#[derive(Debug, Deserialize)]
pub struct SearchQuery {
    pub query: String,
    pub filter: Option<String>,
    pub sort: Option<SortBy>,
    pub page: Option<usize>,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SearchResponse {
    pub contracts: Vec<SearchedContract>,
    pub total: usize,
    pub page: usize,
    pub total_pages: usize,
    pub elapsed_millis: u64,
    pub hits_per_page: usize,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SearchedContract {
    #[serde(flatten)]
    pub contract: Contract,
    pub matching_ranges: HashMap<String, Vec<MatchingRange>>,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct MatchingRange {
    pub start: usize,
    pub end: usize,
}

impl From<meilisearch_sdk::search::MatchRange> for MatchingRange {
    fn from(value: meilisearch_sdk::search::MatchRange) -> Self {
        MatchingRange {
            start: value.start,
            end: value.start + value.length,
        }
    }
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
    let sort: Vec<&str> = vec![sort.to_meilisearch()];

    let filter = query.filter.unwrap_or_default();

    let page = query.page.unwrap_or(1);

    // TODO: make this configurable
    const HITS_PER_PAGE: usize = 20;

    let response = state
        .search(&query.query, &filter, &sort, page, HITS_PER_PAGE)
        .await?;

    debug!("Returning {} results", response.contracts.len());

    // TODO: return formatted results
    Ok(Json(response))
}
