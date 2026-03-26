use std::sync::Arc;

use meilisearch_sdk::{client::Client, indexes::Index, task_info::TaskInfo};

use crate::{Contract, SearchableContract};

#[derive(Debug, Clone)]
pub struct SearchDatabase {
    client: Arc<meilisearch_sdk::client::Client>,
}

type MeilisearchError = meilisearch_sdk::errors::Error;

impl SearchDatabase {
    pub fn new(client: Client) -> Self {
        Self {
            client: Arc::new(client),
        }
    }

    pub fn index(&self) -> Index {
        self.client.index("contracts")
    }

    pub async fn save_contract(&self, contract: Contract) -> Result<TaskInfo, MeilisearchError> {
        let index = self.index();
        let searchable_contract: SearchableContract = contract.into();

        index
            .add_documents(&[searchable_contract], Some("id"))
            .await
    }

    pub fn client(&self) -> &Client {
        &self.client
    }
}
