use anyhow::Context;
use serde::{Serialize, de::DeserializeOwned};

use crate::base_gov::{BaseGovContract, ContractSearchResponse};

const URL: &str = "https://www.base.gov.pt/Base4/pt/resultados/";
const USER_AGENT: &str = "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/58.0.3029.110 Safari/537.3";

#[derive(Debug)]
pub struct ContractSort {
    pub method: ContractSortMethod,
    pub order: SortOrder,
}

#[derive(Debug)]
pub enum SortOrder {
    Ascending,
    Descending,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub enum ContractSortMethod {
    PublicationDate,
    ObjectBriefDescription,
    InitialContractualPrice,
    Id,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase", tag = "type")]
pub enum BaseGovPayload {
    #[serde(rename = "search_contratos")]
    SearchContracts {
        version: &'static str,
        query: &'static str,
        sort: ContractSort,
        page: usize,
        size: usize,
    },
    #[serde(rename = "detail_contratos")]
    ContractDetails { version: &'static str, id: usize },
}

pub struct BaseGovClient {
    client: reqwest::Client,
}

impl BaseGovClient {
    pub fn new() -> Self {
        Self {
            client: reqwest::Client::builder()
                .user_agent(USER_AGENT)
                .build()
                .unwrap(),
        }
    }

    pub async fn fetch_page(
        &self,
        sort: ContractSort,
        page: usize,
        size: usize,
    ) -> anyhow::Result<ContractSearchResponse> {
        let payload = BaseGovPayload::SearchContracts {
            version: "135.0",
            query: "tipo=0&tipocontrato=0&pais=0&distrito=0&concelho=0",
            sort,
            page,
            size,
        };
        self.send_payload(payload).await
    }

    pub async fn get_contract_details(&self, id: usize) -> anyhow::Result<BaseGovContract> {
        let payload = BaseGovPayload::ContractDetails {
            version: "58.0",
            id,
        };
        self.send_payload(payload).await
    }

    async fn send_payload<T: DeserializeOwned>(
        &self,
        payload: BaseGovPayload,
    ) -> anyhow::Result<T> {
        let response = self
            .client
            .post(URL)
            .form(&payload)
            .send()
            .await
            .context("Failed to send POST request")?
            .json::<serde_json::Value>()
            .await
            .context("Failed to parse contracts as JSON")?;

        serde_json::from_value(response)
            .context("Failed to parse JSON response to contract details")
    }
}

impl Serialize for ContractSort {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let order_prefix = match self.order {
            SortOrder::Ascending => "+",
            SortOrder::Descending => "-",
        };

        let method_str = match self.method {
            ContractSortMethod::PublicationDate => "publicationDate",
            ContractSortMethod::ObjectBriefDescription => "objectBriefDescription",
            ContractSortMethod::InitialContractualPrice => "initialContractualPrice",
            ContractSortMethod::Id => "id",
        };

        serializer.serialize_str(&format!("{}{}", order_prefix, method_str))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_get_contract_details() {
        let client = BaseGovClient::new();
        let response = client.get_contract_details(11641358).await;
        assert!(response.is_ok());
    }
}
