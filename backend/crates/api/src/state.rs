use std::{
    collections::HashMap,
    sync::{Arc, RwLock},
};

use anyhow::Context;
use common::{Contract, SearchableContract};
use meilisearch_sdk::settings::{PaginationSetting, Settings};
use serde::Serialize;
use sqlx::PgPool;

use crate::{error::AppResult, filter::Filters, sort::SortField, statistics::Statistics};

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
    pub contract: SearchableContract,
    pub matching_ranges: HashMap<String, Vec<MatchingRange>>,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct MatchingRange {
    pub start: usize,
    pub end: usize,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub indices: Option<Vec<usize>>,
}

impl From<meilisearch_sdk::search::MatchRange> for MatchingRange {
    fn from(value: meilisearch_sdk::search::MatchRange) -> Self {
        MatchingRange {
            start: value.start,
            end: value.start + value.length,
            indices: value.indices,
        }
    }
}

pub struct AppState {
    meilisearch: Arc<meilisearch_sdk::client::Client>,
    pg_pool: PgPool,
    statistics: Arc<RwLock<Statistics>>,
}

impl Clone for AppState {
    fn clone(&self) -> Self {
        Self {
            meilisearch: Arc::clone(&self.meilisearch),
            pg_pool: self.pg_pool.clone(), // PgPool is already an Arc
            statistics: Arc::clone(&self.statistics),
        }
    }
}

impl AppState {
    pub fn new(meilisearch: meilisearch_sdk::client::Client, pg_pool: PgPool) -> Self {
        Self {
            meilisearch: Arc::new(meilisearch),
            pg_pool,
            statistics: Default::default(),
        }
    }

    pub fn get_client(&self) -> Arc<meilisearch_sdk::client::Client> {
        Arc::clone(&self.meilisearch)
    }

    pub fn set_statistics(&self, new_statistics: Statistics) {
        if let Ok(mut statistics) = self.statistics.write() {
            *statistics = new_statistics;
        }
    }

    pub fn get_statistics(&self) -> Statistics {
        self.statistics.read().unwrap().clone()
    }

    pub async fn prepare_settings(&self) -> anyhow::Result<()> {
        let contracts_index = self.meilisearch.index("contracts");

        let settings = Settings::new()
            .with_sortable_attributes(SortField::to_meilisearch_all())
            .with_filterable_attributes(Filters::fields_to_meilisearch_all())
            .with_pagination(PaginationSetting {
                // this is not recommended by meilisearch docs but it is having good performance for now and it is essential for UX
                max_total_hits: 3000000,
            })
            .with_ranking_rules(&[
                "words",
                "typo",
                "proximity",
                "sort",
                "attribute",
                "exactness",
            ]);

        contracts_index
            .set_settings(&settings)
            .await
            .context("Failed to set settings")?;

        Ok(())
    }

    #[tracing::instrument(skip(self))]
    pub async fn search(
        &self,
        query: &str,
        filters: Option<&Filters>,
        sort: &[&str],
        page: usize,
        hits_per_page: usize,
    ) -> AppResult<SearchResponse> {
        let filters = filters.map(Filters::to_meilisearch).unwrap_or_default();
        let filters_ref = filters.iter().map(String::as_str).collect();

        let results = self
            .meilisearch
            .index("contracts")
            .search()
            .with_query(query)
            .with_array_filter(filters_ref)
            .with_sort(sort)
            .with_page(page)
            .with_hits_per_page(hits_per_page)
            .with_show_matches_position(true)
            .execute::<SearchableContract>()
            .await?;

        let contracts = results
            .hits
            .into_iter()
            .map(|hit| SearchedContract {
                contract: hit.result,
                matching_ranges: hit
                    .matches_position
                    .unwrap_or_default()
                    .into_iter()
                    .map(|(field, ranges)| (field, ranges.into_iter().map(Into::into).collect()))
                    .collect(),
            })
            .collect();

        Ok(SearchResponse {
            contracts,
            page: results.page.unwrap_or(0),
            total: results.total_hits.unwrap_or(0),
            total_pages: results.total_pages.unwrap_or(0),
            elapsed_millis: results.processing_time_ms as u64,
            hits_per_page,
        })
    }

    pub async fn get_contract(&self, id: u64) -> AppResult<Option<Contract>> {
        common::db::get_contract(id, &self.pg_pool)
            .await
            .map_err(Into::into)
    }
}
