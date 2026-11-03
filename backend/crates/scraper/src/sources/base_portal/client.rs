use std::time::{Duration, Instant};

use anyhow::Context;
use reqwest::Url;
use serde::{Deserialize, Serialize, de::DeserializeOwned};

use crate::{
    metrics::{self, RequestKind},
    sources::base_portal::BasePortalContract,
};

const URL: &str = "https://www.base.gov.pt/Base4/pt/resultados/";
const USER_AGENT: &str = "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/140.0.0.0 Safari/537.36";

#[derive(Debug, Clone, Copy)]
pub struct ContractSort {
    pub method: ContractSortMethod,
    pub order: SortOrder,
}

#[derive(Debug, Clone, Copy, clap::ValueEnum)]
pub enum SortOrder {
    Ascending,
    Descending,
}

#[derive(Debug, Clone, Copy, Serialize, clap::ValueEnum)]
#[serde(rename_all = "camelCase")]
pub enum ContractSortMethod {
    PublicationDate,
    ObjectBriefDescription,
    InitialContractualPrice,
    Id,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase", tag = "type")]
pub enum BasePortalPayload {
    #[serde(rename = "search_contratos")]
    SearchContracts {
        version: &'static str,
        query: &'static str,
        sort: ContractSort,
        page: usize,
        size: usize,
    },
    #[serde(rename = "detail_contratos")]
    ContractDetails { version: &'static str, id: u64 },
}

/// A minimal representation of the contract.
/// This is used in [BasePortalClient::fetch_page] to only return the contract's ID.
/// This id is then used to fetch the full contract details with [BasePortalClient::get_contract_details].
#[derive(Debug, Deserialize)]
pub struct BasePortalContractMinimal {
    pub id: u64,
}

#[derive(Debug, Deserialize)]
pub struct ContractSearchResponse {
    pub total: usize,
    pub items: Vec<BasePortalContractMinimal>,
}

#[derive(Clone)]
pub struct BasePortalClient {
    client: reqwest::Client,
}

impl BasePortalClient {
    pub fn new(proxy: Option<Url>) -> Self {
        let client = reqwest::Client::builder()
            .user_agent(USER_AGENT)
            .timeout(Duration::from_secs(60));

        let client = match proxy {
            Some(proxy) => client.proxy(reqwest::Proxy::all(proxy).unwrap()),
            None => client,
        };

        let client = client.build().unwrap();

        Self { client }
    }

    pub async fn fetch_page(
        &self,
        sort: ContractSort,
        page: usize,
        size: usize,
    ) -> anyhow::Result<ContractSearchResponse> {
        let payload = BasePortalPayload::SearchContracts {
            version: "140.0",
            query: "tipo=0&tipocontrato=0&pais=0&distrito=0&concelho=0",
            sort,
            page,
            size,
        };
        self.send_payload(payload, RequestKind::Page).await
    }

    /// Returns `None` if the portal doesn't have the contract, which it answers with a record
    /// with empty fields and `id` 0.
    pub async fn get_contract_details(
        &self,
        id: u64,
    ) -> anyhow::Result<Option<BasePortalContract>> {
        let payload = BasePortalPayload::ContractDetails {
            version: "140.0",
            id,
        };
        let value: serde_json::Value = self.send_payload(payload, RequestKind::Details).await?;

        // This record has `null` in some fields that we declare as non-nullable (since it
        // never is on valid records), so we can't parse it as a `BasePortalContract` directly.
        if value["id"] == 0 {
            return Ok(None);
        }

        BasePortalContract::deserialize(&value)
            .map(Some)
            .with_context(|| format!("Failed to parse JSON response to contract details: {value}"))
    }

    async fn send_payload<T: DeserializeOwned>(
        &self,
        payload: BasePortalPayload,
        kind: RequestKind,
    ) -> anyhow::Result<T> {
        let start = Instant::now();
        let response = self.try_send_payload(payload).await;
        metrics::request(kind, response.is_ok(), start.elapsed());
        response
    }

    async fn try_send_payload<T: DeserializeOwned>(
        &self,
        payload: BasePortalPayload,
    ) -> anyhow::Result<T> {
        let response = self
            .client
            .post(URL)
            .form(&payload)
            .send()
            .await
            .context("Failed to send POST request")?;

        let status = response.status();
        let body = response
            .text()
            .await
            .context("Failed to read response body")?;

        let value = serde_json::from_str::<serde_json::Value>(&body).with_context(|| {
            format!("Failed to parse response as JSON (status {status}): {body}")
        })?;

        serde_json::from_value(value).with_context(|| {
            format!("Failed to parse JSON response to contract details (status {status}): {body}")
        })
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
