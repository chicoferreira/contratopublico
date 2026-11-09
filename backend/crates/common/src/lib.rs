use chrono::NaiveDate;
use serde::{Deserialize, Serialize};
use sqlx::Type;

pub mod db;
pub mod searchdb;
pub mod statistics;

#[derive(Debug, Deserialize, Default, Serialize, Clone, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct Contract {
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
    pub execution_places: Vec<String>,
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

#[derive(Debug, Serialize, Deserialize, Clone, Copy, PartialEq, Eq, sqlx::Type)]
#[serde(rename_all = "camelCase")]
#[sqlx(type_name = "contract_type", rename_all = "snake_case")]
pub enum ContractType {
    GoodsAcquisition,
    ServicesAcquisition,
    PublicWorksConcession,
    PublicServicesConcession,
    PublicWorks,
    GoodsLease,
    Partnership,
    Other,
}

impl ContractType {
    pub fn label(self) -> &'static str {
        match self {
            ContractType::GoodsAcquisition => "Aquisição de Bens Móveis",
            ContractType::ServicesAcquisition => "Aquisição de Serviços",
            ContractType::PublicWorksConcession => "Concessão de Obras Públicas",
            ContractType::PublicServicesConcession => "Concessão de Serviços Públicos",
            ContractType::PublicWorks => "Empreitadas de Obras Públicas",
            ContractType::GoodsLease => "Locação de Bens Móveis",
            ContractType::Partnership => "Sociedade",
            ContractType::Other => "Outros",
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone, Copy, PartialEq, Eq, Type)]
#[serde(rename_all = "camelCase")]
#[sqlx(type_name = "procedure_type", rename_all = "snake_case")]
pub enum ProcedureType {
    DirectAward,
    DirectAwardRuralFires,
    SimplifiedDirectAward,
    SimplifiedDirectAwardSpecialMeasures,
    PriorConsultation,
    PriorConsultationRuralFires,
    SimplifiedPriorConsultation,
    PublicTender,
    SimplifiedPublicTender,
    RestrictedTender,
    SimplifiedRestrictedTender,
    NegotiatedProcedure,
    CompetitiveDialogue,
    InnovationPartnership,
    #[sqlx(rename = "framework_agreement_art_258")]
    FrameworkAgreementArt258,
    #[sqlx(rename = "framework_agreement_art_259")]
    FrameworkAgreementArt259,
    GoodsProvision,
    SocialAndOtherSpecificServices,
    SimplifiedDesignContest,
    SimplifiedIdeasContest,
    SpecialSectorsExemption,
    ExcludedContracting,
}

impl ProcedureType {
    pub fn title_and_subtitle(self) -> (&'static str, Option<&'static str>) {
        use ProcedureType::*;
        match self {
            DirectAward => ("Ajuste Direto", None),
            DirectAwardRuralFires => ("Ajuste Direto", Some("Fogos Rurais")),
            SimplifiedDirectAward => ("Ajuste Direto", Some("Simplificado")),
            SimplifiedDirectAwardSpecialMeasures => {
                ("Ajuste Direto", Some("Simplificado Especial"))
            }
            PriorConsultation => ("Consulta Prévia", None),
            PriorConsultationRuralFires => ("Consulta Prévia", Some("Fogos Rurais")),
            SimplifiedPriorConsultation => ("Consulta Prévia", Some("Simplificada")),
            PublicTender => ("Concurso Público", None),
            SimplifiedPublicTender => ("Concurso Público", Some("Simplificado")),
            RestrictedTender => ("Concurso Limitado por Prévia Qualificação", None),
            SimplifiedRestrictedTender => (
                "Concurso Limitado por Prévia Qualificação",
                Some("Simplificado"),
            ),
            NegotiatedProcedure => ("Procedimento de Negociação", None),
            CompetitiveDialogue => ("Diálogo Concorrencial", None),
            InnovationPartnership => ("Parceria para a Inovação", None),
            FrameworkAgreementArt258 => ("Acordo-Quadro", Some("Sem Nova Concorrência")),
            FrameworkAgreementArt259 => ("Acordo-Quadro", Some("Com Nova Concorrência")),
            GoodsProvision => ("Disponibilização de Bens Móveis", None),
            SocialAndOtherSpecificServices => {
                ("Serviços Sociais e Outros Serviços Específicos", None)
            }
            SimplifiedDesignContest => ("Concurso de Conceção", Some("Simplificado")),
            SimplifiedIdeasContest => ("Concurso de Ideias", Some("Simplificado")),
            SpecialSectorsExemption => ("Setores Especiais", Some("Isenção")),
            ExcludedContracting => ("Contratação Excluída", None),
        }
    }
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
#[derive(Debug, Deserialize, Serialize, Clone, PartialEq, Eq, Default)]
pub struct Currency(pub isize);

/// The contract struct that will be saved in meilisearch
/// with only important parameters for faster search
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct SearchableContract {
    pub id: u64,
    pub procedure_type: Option<ProcedureType>,
    // The title and subtitle are stored as strings so they are still searchable
    pub procedure_type_title: Option<String>,
    pub procedure_type_subtitle: Option<String>,
    pub publication_date: NaiveDate,
    pub signing_date: Option<NaiveDate>,
    pub object_brief_description: String,
    pub initial_contractual_price: Currency,
    pub contracting: Vec<Entity>,
    pub contracted: Vec<Entity>,
    pub cpvs: Vec<Cpv>,
    pub regime: Option<String>,
    pub contract_types: Vec<ContractType>,
    // The labels are stored as strings so they are still searchable
    pub contract_type_labels: Vec<String>,
    pub execution_places: Vec<String>,
    pub contract_fundamentation_type: String,
    pub contestants: Vec<Entity>,
    pub invitees: Vec<Entity>,
    pub documents: Vec<Document>,
    pub contracting_procedure_url: Option<String>,
    pub announcement_id: Option<usize>,
}

impl From<Contract> for SearchableContract {
    fn from(contract: Contract) -> Self {
        let (title, subtitle) = contract
            .procedure_type
            .map(ProcedureType::title_and_subtitle)
            .unzip();

        SearchableContract {
            id: contract.id,
            procedure_type: contract.procedure_type,
            procedure_type_title: title.map(str::to_string),
            procedure_type_subtitle: subtitle.flatten().map(str::to_string),
            publication_date: contract.publication_date,
            signing_date: contract.signing_date,
            object_brief_description: contract.object_brief_description,
            initial_contractual_price: contract.initial_contractual_price,
            contracting: contract.contracting,
            contracted: contract.contracted,
            cpvs: contract.cpvs,
            regime: contract.regime,
            contract_type_labels: contract
                .contract_types
                .iter()
                .map(|contract_type| contract_type.label().to_string())
                .collect(),
            contract_types: contract.contract_types,
            execution_places: contract.execution_places,
            contract_fundamentation_type: contract.contract_fundamentation_type,
            contestants: contract.contestants,
            invitees: contract.invitees,
            documents: contract.documents,
            contracting_procedure_url: contract.contracting_procedure_url,
            announcement_id: contract.announcement_id,
        }
    }
}
