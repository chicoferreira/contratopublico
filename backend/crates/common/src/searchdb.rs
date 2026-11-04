use std::{sync::Arc, time::Duration};

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

    pub async fn save_contracts(
        &self,
        contracts: Vec<Contract>,
    ) -> Result<TaskInfo, MeilisearchError> {
        let contracts: Vec<SearchableContract> = contracts.into_iter().map(Into::into).collect();

        self.index().add_documents(&contracts, Some("id")).await
    }

    pub async fn delete_contracts(&self, ids: &[i64]) -> Result<TaskInfo, MeilisearchError> {
        self.index().delete_documents(ids).await
    }

    pub async fn delete_all_contracts(&self) -> Result<TaskInfo, MeilisearchError> {
        self.index().delete_all_documents().await
    }

    /// Waits for the task to finish, and returns its error if it failed.
    pub async fn wait_for_success(
        &self,
        task: TaskInfo,
        timeout: Duration,
    ) -> Result<(), MeilisearchError> {
        let task = task
            .wait_for_completion(&self.client, None, Some(timeout))
            .await?;

        if task.is_failure() {
            return Err(task.unwrap_failure().into());
        }

        Ok(())
    }

    pub fn client(&self) -> &Client {
        &self.client
    }
}
