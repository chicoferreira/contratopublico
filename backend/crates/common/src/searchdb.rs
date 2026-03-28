use std::sync::Arc;

use meilisearch_sdk::{client::Client, indexes::Index, task_info::TaskInfo};

use crate::{Contract, SearchableContract};

#[derive(clap::Parser)]
pub struct MeilisearchConfig {
    #[clap(long, env, default_value = "http://localhost:7700")]
    pub meilisearch_url: String,
    #[clap(long, env = "MEILI_MASTER_KEY", default_value = "masterKey")]
    pub meilisearch_api_key: Option<String>,
}

impl MeilisearchConfig {
    pub fn create_client(&self) -> Result<Client, meilisearch_sdk::errors::Error> {
        meilisearch_sdk::client::Client::new(
            self.meilisearch_url.clone(),
            self.meilisearch_api_key.clone(),
        )
    }
}

#[derive(Debug, Clone)]
pub struct SearchDatabase {
    client: Arc<Client>,
}

type MeilisearchError = meilisearch_sdk::errors::Error;

impl SearchDatabase {
    pub fn new_from_config(config: MeilisearchConfig) -> Result<Self, MeilisearchError> {
        Ok(Self::new(config.create_client()?))
    }

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
