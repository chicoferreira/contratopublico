use chrono::NaiveDate;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Filters {
    #[serde(default)]
    pub min_id: Option<u64>,
    #[serde(default)]
    pub max_id: Option<u64>,
    #[serde(default)]
    pub start_publication_date: Option<NaiveDate>,
    #[serde(default)]
    pub end_publication_date: Option<NaiveDate>,
    #[serde(default)]
    pub start_signing_date: Option<NaiveDate>,
    #[serde(default)]
    pub end_signing_date: Option<NaiveDate>,
    #[serde(default)]
    pub contracted: Option<String>,
    #[serde(default)]
    pub contracting: Option<String>,
    #[serde(default)]
    pub min_price: Option<i64>,
    #[serde(default)]
    pub max_price: Option<i64>,
}

impl Filters {
    pub fn fields_to_meilisearch_all() -> Vec<&'static str> {
        vec![
            "id",
            "publicationDate",
            "signingDate",
            "contracted",
            "contracting",
            "initialContractualPrice",
        ]
    }

    pub fn to_meilisearch(&self) -> Vec<String> {
        let mut filters = Vec::new();

        if let Some(id) = self.min_id {
            filters.push(format!("id >= {id}"));
        }
        if let Some(id) = self.max_id {
            filters.push(format!("id <= {id}"));
        }
        if let Some(start_date) = self.start_publication_date {
            filters.push(format!("publicationDate >= '{start_date}'"));
        }
        if let Some(end_date) = self.end_publication_date {
            filters.push(format!("publicationDate <= '{end_date}'"));
        }
        if let Some(start_date) = self.start_signing_date {
            filters.push(format!("signingDate >= '{start_date}'"));
        }
        if let Some(end_date) = self.end_signing_date {
            filters.push(format!("signingDate <= '{end_date}'"));
        }
        if let Some(entity) = &self.contracted {
            filters.push(format!("contracted = '{entity}'"));
        }
        if let Some(entity) = &self.contracting {
            filters.push(format!("contracting = '{entity}'"));
        }
        if let Some(price) = self.min_price {
            filters.push(format!("initialContractualPrice >= {price}"));
        }
        if let Some(price) = self.max_price {
            filters.push(format!("initialContractualPrice <= {price}"));
        }

        filters
    }
}
