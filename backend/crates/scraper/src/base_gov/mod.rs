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
    /// The date when the contract was signed.
    pub signing_date: Option<NaiveDate>,

    #[serde(deserialize_with = "de::deserialize_date")]
    /// The date when the contract was published.
    pub publication_date: NaiveDate,

    #[serde(deserialize_with = "de::deserialize_euros")]
    /// The initial contractual price of the contract.
    pub initial_contractual_price: Currency,

    // TODO: Add documentation here
    pub regime: String,

    /// The status of the contract. (idk what status mean)
    pub contract_status: Option<String>,

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

    /// Entities that have been invited to participate in the contract.
    pub invitees: Vec<BaseGovEntity>,

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

    /// The type of end of contract (ex: Cumprimento integral do contrato)
    pub end_of_contract_type: Option<String>,

    #[serde(deserialize_with = "de::deserialize_optional_date")]
    /// The date when the contract was complete or closed.
    pub close_date: Option<NaiveDate>,

    #[serde(deserialize_with = "de::deserialize_optional_euros")]
    /// The price of the contract when it was completed or closed.
    pub total_effective_price: Option<Currency>,

    /// What caused the deadline to change (if changed)
    pub causes_deadline_change: Option<String>,

    /// What caused the price to change (if changed)
    pub causes_price_change: Option<String>,
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

impl From<BaseGovContract> for Contract {
    fn from(contract: BaseGovContract) -> Contract {
        Contract {
            id: contract.id,
            contracting_procedure_type: contract.contracting_procedure_type,
            publication_date: contract.publication_date,
            signing_date: contract.signing_date,
            ccp: contract.ccp,
            contracted: contract.contracted.into_iter().map(Into::into).collect(),
            contracting: contract.contracting.into_iter().map(Into::into).collect(),
            object_brief_description: contract.object_brief_description,
            initial_contractual_price: contract.initial_contractual_price,
            description: contract.description,
            cpv: contract.cpv.into(),
            regime: contract.regime,
            contract_status: contract.contract_status,
            non_written_contract_justification_types: contract
                .non_written_contract_justification_types,
            contract_types: contract.contract_types,
            execution_deadline_days: contract.execution_deadline_days,
            execution_place: contract.execution_place,
            contract_fundamentation_type: contract.contract_fundamentation_type,
            contestants: contract.contestants.into_iter().map(Into::into).collect(),
            invitees: contract.invitees.into_iter().map(Into::into).collect(),
            documents: contract.documents.into_iter().map(Into::into).collect(),
            contracting_procedure_url: contract.contracting_procedure_url,
            announcement_id: contract.announcement_id,
            direct_award_fundamentation_type: contract.direct_award_fundamentation_type,
            observations: contract.observations,
            end_of_contract_type: contract.end_of_contract_type,
            close_date: contract.close_date,
            total_effective_price: contract.total_effective_price,
            causes_deadline_change: contract.causes_deadline_change,
            causes_price_change: contract.causes_price_change,
        }
    }
}

impl From<BaseGovEntity> for Entity {
    fn from(entity: BaseGovEntity) -> Entity {
        Entity {
            id: entity.id,
            description: entity.description,
            nif: entity.nif,
        }
    }
}

impl From<BaseGovCpv> for Cpv {
    fn from(cpv: BaseGovCpv) -> Cpv {
        Cpv {
            code: cpv.code,
            designation: cpv.designation,
        }
    }
}

impl From<BaseGovDocument> for Document {
    fn from(document: BaseGovDocument) -> Document {
        Document {
            id: document.id,
            description: document.description,
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
