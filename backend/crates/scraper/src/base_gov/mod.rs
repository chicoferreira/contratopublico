pub mod client;
mod de;

use chrono::NaiveDate;
use common::{Contract, Cpv, Currency, Document, Entity};
use serde::Deserialize;

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BaseGovContract {
    /// Unique identifier of the contract.
    pub id: usize,

    /// Description of the contract.
    pub description: Option<String>,

    /// A brief description of the contract. Used as a title of the contract.
    pub object_brief_description: String,

    /// Procedure type of the contract. (Concurso público, Ajuste direto, etc)
    pub contracting_procedure_type: String,

    /// The entities responsible for contracting the contracted.
    pub contracting: Vec<BaseGovEntity>,

    /// The entities contracted for the contract.
    pub contracted: Vec<BaseGovEntity>,

    #[serde(flatten)]
    pub cpv: BaseGovCpv,

    #[serde(deserialize_with = "de::deserialize_optional_date")]
    // The date when the contract was signed.
    pub signing_date: Option<NaiveDate>,

    #[serde(deserialize_with = "de::deserialize_date")]
    // The date when the contract was published.
    pub publication_date: NaiveDate,

    #[serde(deserialize_with = "de::deserialize_euros")]
    /// The initial contractual price of the contract.
    pub initial_contractual_price: Currency,

    // TODO: Add documentation here
    pub regime: String,

    // TODO: Check what this value represents
    pub contract_status: Option<usize>,

    // TODO: Check what this value represents
    pub non_written_contract_justification_types: String,

    // TODO: Add documentation here
    pub contract_types: String,

    #[serde(
        rename = "executionDeadline",
        deserialize_with = "de::deserialize_execution_deadline"
    )]
    // TODO: Add documentation here
    pub execution_deadline_days: usize,

    /// The place where the contract will be executed.
    pub execution_place: String,

    // TODO: Add documentation here
    pub contract_fundamentation_type: String,

    /// The contestants involved in the contract.
    pub contestants: Vec<BaseGovEntity>,

    // TODO: Check what this value represents
    pub invitees: Vec<isize>,

    /// The documents related to the contract.
    pub documents: Vec<BaseGovDocument>,

    /// The URL with information about the contracting procedure.
    pub contracting_procedure_url: Option<String>,

    #[serde(deserialize_with = "de::deserialize_announcement_id")]
    /// The internal Portal BASE identifier of the announcement related to this contract.
    pub announcement_id: Option<usize>,

    // TODO: Check what this value represents
    pub direct_award_fundamentation_type: String,

    // TODO: Check what this value represents
    pub observations: Option<String>,

    // TODO: Check what this value represents
    pub ccp: bool,
}

/// CPV (Common Procurement Vocabulary) identifies the type of goods or services being contracted.
/// For example: "48000000-8" = "Pacotes de software e sistemas de informação"
#[derive(Debug, Deserialize)]
pub struct BaseGovCpv {
    #[serde(rename = "cpvs")]
    pub code: String,
    #[serde(rename = "cpvsDesignation")]
    pub designation: String,
}

#[derive(Debug, Deserialize)]
pub struct BaseGovEntity {
    /// The internal Portal BASE identifier
    pub id: usize,
    /// The NIF of the entity
    pub nif: String,
    /// The name/description of the entity
    pub description: String,
}

#[derive(Debug, Deserialize)]
pub struct BaseGovDocument {
    /// The internal Portal BASE identifier
    pub id: usize,
    /// The file name of the document
    pub description: String,
}

impl Into<Contract> for BaseGovContract {
    fn into(self) -> Contract {
        Contract {
            id: self.id,
            contracting_procedure_type: self.contracting_procedure_type,
            publication_date: self.publication_date,
            signing_date: self.signing_date,
            ccp: self.ccp,
            contracted: self.contracted.into_iter().map(Into::into).collect(),
            contracting: self.contracting.into_iter().map(Into::into).collect(),
            object_brief_description: self.object_brief_description,
            initial_contractual_price: self.initial_contractual_price,
            description: self.description,
            cpv: self.cpv.into(),
            regime: self.regime,
            contract_status: self.contract_status,
            non_written_contract_justification_types: self.non_written_contract_justification_types,
            contract_types: self.contract_types,
            execution_deadline_days: self.execution_deadline_days,
            execution_place: self.execution_place,
            contract_fundamentation_type: self.contract_fundamentation_type,
            contestants: self.contestants.into_iter().map(Into::into).collect(),
            invitees: self.invitees,
            documents: self.documents.into_iter().map(Into::into).collect(),
            contracting_procedure_url: self.contracting_procedure_url,
            announcement_id: self.announcement_id,
            direct_award_fundamentation_type: self.direct_award_fundamentation_type,
            observations: self.observations,
        }
    }
}

impl Into<Entity> for BaseGovEntity {
    fn into(self) -> Entity {
        Entity {
            id: self.id,
            description: self.description,
            nif: self.nif,
        }
    }
}

impl Into<Cpv> for BaseGovCpv {
    fn into(self) -> Cpv {
        Cpv {
            code: self.code,
            designation: self.designation,
        }
    }
}

impl Into<Document> for BaseGovDocument {
    fn into(self) -> Document {
        Document {
            id: self.id,
            description: self.description,
        }
    }
}

/// A minimal representation of the contract.
/// This is used in [BaseGovClient::search_contracts] to only return the contract's ID.
/// This id is then used to fetch the full contract details with [BaseGovClient::get_contract_details].
#[derive(Debug, Deserialize)]
pub struct BaseGovContractMinimal {
    pub id: usize,
}

#[derive(Debug, Deserialize)]
pub struct ContractSearchResponse {
    pub total: usize,
    pub items: Vec<BaseGovContractMinimal>,
}
