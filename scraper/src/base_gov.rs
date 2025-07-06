use common::Contract;
use serde::{Deserialize, Serialize};

const URL: &str = "https://www.base.gov.pt/Base4/pt/resultados/";
const USER_AGENT: &str = "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/58.0.3029.110 Safari/537.3";

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
pub struct BaseGovPayload {
    #[serde(rename = "type")]
    pub payload_type: String,
    pub version: String,
    pub query: String,
    pub sort: ContractSort,
    pub page: usize,
    pub size: usize,
}

impl BaseGovPayload {
    pub fn new_search_all_contracts(sort: ContractSort, page: usize, size: usize) -> Self {
        Self {
            payload_type: "search_contratos".to_string(),
            version: "91.0".to_string(),
            query: "tipo=0&tipocontrato=0&pais=0&distrito=0&concelho=0".to_string(),
            sort,
            page,
            size,
        }
    }
}

#[derive(Debug, Deserialize)]
pub struct ContractSearchResponse {
    pub total: usize,
    pub items: Vec<Contract>,
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

    pub async fn search_contracts(
        &self,
        payload: BaseGovPayload,
    ) -> Result<ContractSearchResponse, reqwest::Error> {
        let response = self.client.post(URL).form(&payload).send().await?;
        response.json::<ContractSearchResponse>().await
    }
}
