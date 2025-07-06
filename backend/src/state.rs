use std::sync::Arc;

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

    pub async fn search(&self, query: &str) -> AppResult<SearchResults<Contract>> {
        self.meilisearch
            .index("contracts")
            .search()
            .with_query(query)
            .execute::<Contract>()
            .await
            .map_err(AppError::MeilisearchError)
    }
}
