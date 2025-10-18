use chrono::NaiveDate;
use serde::{Deserialize, Serialize};

pub mod db;

#[derive(Debug, Deserialize, Serialize, Clone, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct Contract {
    pub id: u64,
    pub contracting_procedure_type: String,
    pub publication_date: NaiveDate,
    pub signing_date: Option<NaiveDate>,
    pub ccp: bool,
    pub object_brief_description: String,
    pub initial_contractual_price: Currency,
    pub description: Option<String>,
    pub contracting: Vec<Entity>,
    pub contracted: Vec<Entity>,
    pub cpvs: Vec<Cpv>,
    pub regime: Option<String>,
    pub contract_status: Option<String>,
    pub non_written_contract_justification_types: String,
    pub contract_types: String,
    pub execution_deadline_days: usize,
    pub execution_place: String,
    pub contract_fundamentation_type: String,
    pub contestants: Vec<Entity>,
    pub invitees: Vec<Entity>,
    pub documents: Vec<Document>,
    pub contracting_procedure_url: Option<String>,
    pub announcement_id: Option<usize>,
    pub direct_award_fundamentation_type: String,
    pub observations: Option<String>,
    pub end_of_contract_type: Option<String>,
    pub close_date: Option<NaiveDate>,
    pub total_effective_price: Option<Currency>,
    pub causes_deadline_change: Option<String>,
    pub causes_price_change: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq)]
pub struct Cpv {
    pub code: String,
    pub designation: String,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq)]
pub struct Entity {
    pub id: u64,
    pub nif: String,
    pub description: String,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq)]
pub struct Document {
    pub id: u64,
    pub description: String,
}

/// A currency value that is represented as a `isize`.
/// The last two digits always represent cents.
#[derive(Debug, Deserialize, Serialize, Clone, PartialEq, Eq)]
pub struct Currency(pub isize);

/// The contract struct that will be saved in meilisearch
/// with only important parameters for faster search
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct SearchableContract {
    pub id: u64,
    pub contracting_procedure_type: String,
    pub publication_date: NaiveDate,
    pub signing_date: Option<NaiveDate>,
    pub object_brief_description: String,
    pub initial_contractual_price: Currency,
    pub contracting: Vec<Entity>,
    pub contracted: Vec<Entity>,
    pub cpvs: Vec<Cpv>,
    pub regime: Option<String>,
    pub contract_types: String,
    pub execution_place: String,
    pub contract_fundamentation_type: String,
    pub contestants: Vec<Entity>,
    pub invitees: Vec<Entity>,
    pub documents: Vec<Document>,
    pub contracting_procedure_url: Option<String>,
    pub announcement_id: Option<usize>,
}

impl From<Contract> for SearchableContract {
    fn from(contract: Contract) -> Self {
        SearchableContract {
            id: contract.id,
            contracting_procedure_type: contract.contracting_procedure_type,
            publication_date: contract.publication_date,
            signing_date: contract.signing_date,
            object_brief_description: contract.object_brief_description,
            initial_contractual_price: contract.initial_contractual_price,
            contracting: contract.contracting,
            contracted: contract.contracted,
            cpvs: contract.cpvs,
            regime: contract.regime,
            contract_types: contract.contract_types,
            execution_place: contract.execution_place,
            contract_fundamentation_type: contract.contract_fundamentation_type,
            contestants: contract.contestants,
            invitees: contract.invitees,
            documents: contract.documents,
            contracting_procedure_url: contract.contracting_procedure_url,
            announcement_id: contract.announcement_id,
        }
    }
}
