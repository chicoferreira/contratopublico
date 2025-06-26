use chrono::NaiveDate;
use serde::{Deserialize, Serialize};

const URL: &str = "https://www.base.gov.pt/Base4/pt/resultados/";

#[derive(Debug, Serialize)]
struct BaseGovPayload {
    #[serde(rename = "type")]
    payload_type: String,
    version: String,
    query: String,
    sort: String,
    page: usize,
    size: usize,
}

#[derive(Debug, Deserialize)]
struct BaseGovResponse {
    total: usize,
    items: Vec<BaseGovItem>,
}

fn deserialize_date<'de, D>(deserializer: D) -> Result<NaiveDate, D::Error>
where
    D: serde::Deserializer<'de>,
{
    let date_str: String = Deserialize::deserialize(deserializer)?;
    NaiveDate::parse_from_str(&date_str, "%d-%m-%Y").map_err(serde::de::Error::custom)
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct BaseGovItem {
    id: usize,
    contracting_procedure_type: String,
    #[serde(deserialize_with = "deserialize_date")]
    publication_date: NaiveDate,
    #[serde(deserialize_with = "deserialize_date")]
    signing_date: NaiveDate,
    ccp: bool,
    contracted: String,
    contracting: String,
    object_brief_description: String,
    initial_contractual_price: String,
}

#[tokio::main]
async fn main() {
    let client = reqwest::Client::builder()
        .user_agent("Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/58.0.3029.110 Safari/537.3")
        .build()
        .unwrap();

    let payload = BaseGovPayload {
        payload_type: "search_contratos".to_string(),
        version: "127.0".to_string(),
        query: "tipo=0&tipocontrato=0&pais=0&distrito=0&concelho=0".to_string(),
        sort: "+initialContractualPrice".to_string(),
        page: 0,
        size: 50,
    };

    let response = client
        .post(URL)
        .form(&payload)
        .send()
        .await
        .unwrap()
        .json::<BaseGovResponse>()
        .await
        .unwrap();

    println!("Total results: {:#?}", response.total);
}
