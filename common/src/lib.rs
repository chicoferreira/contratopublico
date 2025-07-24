use chrono::NaiveDate;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize, Clone, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct Contract {
    pub id: usize,
    pub contracting_procedure_type: String,
    pub publication_date: NaiveDate,
    pub signing_date: Option<NaiveDate>,
    pub ccp: bool,
    pub contracted: String,
    pub contracting: String,
    pub object_brief_description: String,
    pub initial_contractual_price: Currency,
}

/// A currency value that is represented as a `isize`.
/// The last two digits always represent cents.
#[derive(Debug, Deserialize, Serialize, Clone, PartialEq, Eq)]
pub struct Currency(pub isize);
