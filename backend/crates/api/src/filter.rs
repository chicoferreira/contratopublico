use chrono::NaiveDate;
use common::{ContractType, Cpv, ProcedureType};
use garde::Validate;
use itertools::Itertools;
use serde::{Deserialize, Serialize};
use serde_json::json;

#[derive(Debug, Deserialize, Default, Validate)]
#[serde(rename_all = "camelCase")]
#[garde(allow_unvalidated)]
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
    #[garde(length(chars, max = 256))]
    pub contracted: Option<String>,
    #[serde(default)]
    #[garde(length(chars, max = 256))]
    pub contracting: Option<String>,
    #[serde(default)]
    pub min_price: Option<i64>,
    #[serde(default)]
    pub max_price: Option<i64>,
    #[serde(default)]
    #[garde(length(max = 8))]
    pub contract_types: Vec<ContractType>,
    #[serde(default)]
    #[garde(length(max = 22))]
    pub procedure_types: Vec<ProcedureType>,
    #[serde(default)]
    #[garde(length(max = 50))]
    pub cpvs: Vec<String>,
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
            "contractTypes",
            "procedureType",
            "cpvs.code",
        ]
    }

    fn escape_string_value(value: &str) -> String {
        value.replace('\'', "\\'")
    }

    fn in_filter<T: Serialize>(field: &str, values: &[T]) -> String {
        let values = values.iter().map(|v| json!(v)).join(", ");
        format!("{field} IN [{values}]")
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
        for (field, entity) in [
            ("contracted", &self.contracted),
            ("contracting", &self.contracting),
        ] {
            if let Some(entity) = entity {
                let escaped = Self::escape_string_value(entity);
                filters.push(format!(
                    "{field}.description = '{escaped}' OR {field}.nif = '{escaped}'"
                ));
            }
        }
        if let Some(price) = self.min_price {
            filters.push(format!("initialContractualPrice >= {price}"));
        }
        if let Some(price) = self.max_price {
            filters.push(format!("initialContractualPrice <= {price}"));
        }
        if !self.contract_types.is_empty() {
            filters.push(Self::in_filter("contractTypes", &self.contract_types));
        }
        if !self.procedure_types.is_empty() {
            filters.push(Self::in_filter("procedureType", &self.procedure_types));
        }
        // Unknown codes are skipped. Users can edit the filters in the URL,
        // and rejecting them would force the frontend to handle that error, which we don't.
        let cpv_filter = self
            .cpvs
            .iter()
            .filter_map(|code| Cpv::from_code(code))
            .map(|cpv| format!("cpvs.code STARTS WITH '{}'", cpv.stem()))
            .join(" OR ");
        if !cpv_filter.is_empty() {
            filters.push(cpv_filter);
        }

        filters
    }
}
