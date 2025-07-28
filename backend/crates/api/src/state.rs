use std::sync::Arc;

use anyhow::Context;
use common::Contract;
use meilisearch_sdk::{
    client::Client,
    settings::{PaginationSetting, Settings},
};

use crate::{
    search::{SearchResponse, SearchedContract},
    sort::SortField,
};

pub struct AppState {
    meilisearch: Arc<Client>,
}

impl Clone for AppState {
    fn clone(&self) -> Self {
        Self {
            meilisearch: Arc::clone(&self.meilisearch),
        }
    }
}

pub enum AppError {
    MeilisearchError(meilisearch_sdk::errors::Error),
}

pub type AppResult<T> = Result<T, AppError>;

impl AppState {
    pub fn new(meilisearch: Client) -> Self {
        Self {
            meilisearch: Arc::new(meilisearch),
        }
    }

    pub fn get_client(&self) -> Arc<Client> {
        Arc::clone(&self.meilisearch)
    }

    pub async fn prepare_settings(&self) -> anyhow::Result<()> {
        let contracts_index = self.meilisearch.index("contracts");

        let settings = Settings::new()
            .with_sortable_attributes(SortField::to_meilisearch_all())
            .with_filterable_attributes([
                "contractingProcedureType",
                "ccp",
                "contracted",
                "contracting",
            ])
            .with_pagination(PaginationSetting {
                // this is not recommended by meilisearch docs but it is having good performance for now and it is essential for UX
                max_total_hits: 3000000,
            });

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
        filter: &str,
        sort: &[&str],
        page: usize,
        hits_per_page: usize,
    ) -> AppResult<SearchResponse> {
        let results = self
            .meilisearch
            .index("contracts")
            .search()
            .with_query(query)
            .with_filter(filter)
            .with_sort(sort)
            .with_page(page)
            .with_hits_per_page(hits_per_page)
            .with_show_matches_position(true)
            .execute::<Contract>()
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
