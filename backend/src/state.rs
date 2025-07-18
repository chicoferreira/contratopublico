use std::sync::Arc;

use anyhow::Context;
use common::Contract;
use meilisearch_sdk::{client::Client, search::SearchResults};

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

    pub async fn prepare_indexes(&self) -> anyhow::Result<()> {
        let contracts_index = self.meilisearch.index("contracts");

        contracts_index
            .set_sortable_attributes([
                "publicationDate",
                "signingDate",
                "initialContractualPrice",
                "id",
            ])
            .await
            .context("Failed to set sortable attributes")?;

        contracts_index
            .set_filterable_attributes([
                "contractingProcedureType",
                "ccp",
                "contracted",
                "contracting",
            ])
            .await
            .context("Failed to set filterable attributes")?;

        Ok(())
    }

    #[tracing::instrument(skip(self))]
    pub async fn search(
        &self,
        query: &str,
        filter: &str,
        sort: &[&str],
        page: usize,
        offset: usize,
        hits_per_page: usize,
    ) -> AppResult<SearchResults<Contract>> {
        self.meilisearch
            .index("contracts")
            .search()
            .with_query(query)
            .with_page(page)
            .with_offset(offset)
            .with_filter(filter)
            .with_sort(sort)
            .with_hits_per_page(hits_per_page)
            .execute::<Contract>()
            .await
            .map_err(AppError::MeilisearchError)
    }
}
