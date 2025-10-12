use std::sync::{Arc, RwLock};

use anyhow::Context;
use common::SearchableContract;
use meilisearch_sdk::{
    client::Client,
    settings::{PaginationSetting, Settings},
};

use crate::{
    filter::Filters,
    search::{SearchResponse, SearchedContract},
    sort::SortField,
    statistics::Statistics,
};

pub struct AppState {
    meilisearch: Arc<Client>,
    statistics: Arc<RwLock<Statistics>>,
}

impl Clone for AppState {
    fn clone(&self) -> Self {
        Self {
            meilisearch: Arc::clone(&self.meilisearch),
            statistics: Arc::clone(&self.statistics),
        }
    }
}

pub enum AppError {
    MeilisearchError(meilisearch_sdk::errors::Error),
    JsonParseError(String),
}

pub type AppResult<T> = Result<T, AppError>;

impl AppState {
    pub fn new(meilisearch: Client) -> Self {
        Self {
            meilisearch: Arc::new(meilisearch),
            statistics: Default::default(),
        }
    }

    pub fn get_client(&self) -> Arc<Client> {
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
            .await
            .map_err(AppError::MeilisearchError)?;

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
}
