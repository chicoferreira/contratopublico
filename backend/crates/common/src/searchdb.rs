use std::sync::Arc;

use crate::{Contract, SearchableContract};

#[derive(Debug, Clone)]
pub struct SearchDatabase {
    client: Arc<meilisearch_sdk::client::Client>,
}

impl SearchDatabase {
    pub fn new(client: meilisearch_sdk::client::Client) -> Self {
        Self {
            client: Arc::new(client),
        }
    }

    pub fn index(&self) -> meilisearch_sdk::indexes::Index {
        self.client.index("contracts")
    }

    pub async fn save_contract(
        &self,
        contract: Contract,
    ) -> Result<meilisearch_sdk::task_info::TaskInfo, meilisearch_sdk::errors::Error> {
        let index = self.index();
        let searchable_contract: SearchableContract = contract.into();

        index
            .add_documents(&[searchable_contract], Some("id"))
            .await
    }
}
