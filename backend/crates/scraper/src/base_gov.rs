use anyhow::Context;
use chrono::NaiveDate;
use common::{Contract, Currency};
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
#[serde(rename_all = "camelCase")]
pub struct BaseGovContract {
    pub id: usize,
    pub contracting_procedure_type: String,
    #[serde(deserialize_with = "deserialize_date")]
    pub publication_date: NaiveDate,
    #[serde(deserialize_with = "deserialize_optional_date")]
    pub signing_date: Option<NaiveDate>,
    pub ccp: bool,
    pub contracted: String,
    pub contracting: String,
    pub object_brief_description: String,
    #[serde(deserialize_with = "deserialize_euros")]
    pub initial_contractual_price: Currency,
}

impl Into<Contract> for BaseGovContract {
    fn into(self) -> Contract {
        Contract {
            id: self.id,
            contracting_procedure_type: self.contracting_procedure_type,
            publication_date: self.publication_date,
            signing_date: self.signing_date,
            ccp: self.ccp,
            contracted: self.contracted,
            contracting: self.contracting,
            object_brief_description: self.object_brief_description,
            initial_contractual_price: self.initial_contractual_price,
        }
    }
}

fn deserialize_date<'de, D>(deserializer: D) -> Result<NaiveDate, D::Error>
where
    D: serde::Deserializer<'de>,
{
    let date_str: String = Deserialize::deserialize(deserializer)?;
    NaiveDate::parse_from_str(&date_str, "%d-%m-%Y").map_err(serde::de::Error::custom)
}

fn deserialize_optional_date<'de, D>(deserializer: D) -> Result<Option<NaiveDate>, D::Error>
where
    D: serde::Deserializer<'de>,
{
    let date_str: Option<String> = Deserialize::deserialize(deserializer)?;
    Ok(date_str
        .map(|date_str| {
            NaiveDate::parse_from_str(&date_str, "%d-%m-%Y").map_err(serde::de::Error::custom)
        })
        .transpose()?)
}

fn deserialize_euros<'de, D>(deserializer: D) -> Result<Currency, D::Error>
where
    D: serde::Deserializer<'de>,
{
    // 5.611,10 €
    let euros_str: String = Deserialize::deserialize(deserializer)?;
    let euros_str = euros_str
        .trim_start()
        .trim_end_matches(" €")
        .replace(".", "")
        .replace(",", "");

    let big_int: isize = euros_str.parse().map_err(serde::de::Error::custom)?;

    Ok(Currency(big_int))
}

#[derive(Debug, Deserialize)]
pub struct ContractSearchResponse {
    pub total: usize,
    pub items: Vec<BaseGovContract>,
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
    ) -> anyhow::Result<ContractSearchResponse> {
        let response = self
            .client
            .post(URL)
            .form(&payload)
            .send()
            .await
            .context("Failed to send POST request")?;

        response
            .json::<ContractSearchResponse>()
            .await
            .context("Failed to parse contracts")
    }
}
