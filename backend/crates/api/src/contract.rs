use chrono::NaiveDate;
use common::{
    Contract, ContractType, Cpv, Currency, DetailedExecutionPlace, Document, Entity, ProcedureType,
};
use serde::Serialize;

/// A contract as shown on its page, with CPV designations and execution place labels.
#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ContractDetails {
    pub id: u64,
    pub procedure_type: Option<ProcedureType>,
    pub publication_date: NaiveDate,
    pub signing_date: Option<NaiveDate>,
    pub object_brief_description: String,
    pub initial_contractual_price: Currency,
    pub description: Option<String>,
    pub contracting: Vec<Entity>,
    pub contracted: Vec<Entity>,
    pub cpvs: Vec<Cpv>,
    pub regime: Option<String>,
    pub contract_status: Option<String>,
    pub non_written_contract_justification_types: Vec<String>,
    pub contract_types: Vec<ContractType>,
    pub execution_deadline_days: usize,
    pub execution_places: Vec<DetailedExecutionPlace>,
    pub contract_fundamentation_type: String,
    pub contestants: Vec<Entity>,
    pub invitees: Vec<Entity>,
    pub documents: Vec<Document>,
    pub contracting_procedure_url: Option<String>,
    pub announcement_id: Option<usize>,
    pub direct_award_fundamentation_type: Option<String>,
    pub observations: Option<String>,
    pub end_of_contract_type: Option<String>,
    pub close_date: Option<NaiveDate>,
    pub total_effective_price: Option<Currency>,
    pub causes_deadline_change: Option<String>,
    pub causes_price_change: Option<String>,
    pub estimated_base_price: Option<Currency>,
    pub award_decision_date: Option<NaiveDate>,
    pub framework_agreement_description: Option<String>,
    pub special_measures: Option<String>,
    pub ambient_criteria: bool,
}

impl From<Contract> for ContractDetails {
    fn from(contract: Contract) -> Self {
        ContractDetails {
            id: contract.id,
            procedure_type: contract.procedure_type,
            publication_date: contract.publication_date,
            signing_date: contract.signing_date,
            object_brief_description: contract.object_brief_description,
            initial_contractual_price: contract.initial_contractual_price,
            description: contract.description,
            contracting: contract.contracting,
            contracted: contract.contracted,
            cpvs: contract.cpvs.into_iter().map(Cpv::from).collect(),
            regime: contract.regime,
            contract_status: contract.contract_status,
            non_written_contract_justification_types: contract
                .non_written_contract_justification_types,
            contract_types: contract.contract_types,
            execution_deadline_days: contract.execution_deadline_days,
            execution_places: contract
                .execution_places
                .into_iter()
                .map(DetailedExecutionPlace::from)
                .collect(),
            contract_fundamentation_type: contract.contract_fundamentation_type,
            contestants: contract.contestants,
            invitees: contract.invitees,
            documents: contract.documents,
            contracting_procedure_url: contract.contracting_procedure_url,
            announcement_id: contract.announcement_id,
            direct_award_fundamentation_type: contract.direct_award_fundamentation_type,
            observations: contract.observations,
            end_of_contract_type: contract.end_of_contract_type,
            close_date: contract.close_date,
            total_effective_price: contract.total_effective_price,
            causes_deadline_change: contract.causes_deadline_change,
            causes_price_change: contract.causes_price_change,
            estimated_base_price: contract.estimated_base_price,
            award_decision_date: contract.award_decision_date,
            framework_agreement_description: contract.framework_agreement_description,
            special_measures: contract.special_measures,
            ambient_criteria: contract.ambient_criteria,
        }
    }
}
