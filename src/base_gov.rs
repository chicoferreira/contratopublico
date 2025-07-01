use chrono::NaiveDate;
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

fn serialize_date<S>(date: &NaiveDate, serializer: S) -> Result<S::Ok, S::Error>
where
    S: serde::Serializer,
{
    serializer.serialize_str(&date.format("%d-%m-%Y").to_string())
}

fn serialize_optional_date<S>(date: &Option<NaiveDate>, serializer: S) -> Result<S::Ok, S::Error>
where
    S: serde::Serializer,
{
    match date {
        Some(d) => serializer.serialize_str(&d.format("%d-%m-%Y").to_string()),
        None => serializer.serialize_none(),
    }
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Contract {
    pub id: usize,
    pub contracting_procedure_type: String,
    #[serde(
        deserialize_with = "deserialize_date",
        serialize_with = "serialize_date"
    )]
    pub publication_date: NaiveDate,
    #[serde(
        deserialize_with = "deserialize_optional_date",
        serialize_with = "serialize_optional_date"
    )]
    pub signing_date: Option<NaiveDate>,
    pub ccp: bool,
    pub contracted: String,
    pub contracting: String,
    pub object_brief_description: String,
    pub initial_contractual_price: String,
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
