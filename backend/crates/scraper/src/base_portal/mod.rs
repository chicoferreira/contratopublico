//! Scraper for the public Portal BASE website at <https://www.base.gov.pt/base4>.

pub mod client;
mod convert;
pub mod throttled_client;

use serde::Deserialize;

/// A contract from the Portal BASE website.
#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BasePortalContract {
    /// The contract id, which is the same in the API and the portal.
    /// Example: 9497676
    pub id: u64,

    /// The description of the contract.
    /// Example: "Formação em Contratação Pública NRO.2017/00732"
    pub description: Option<String>,

    /// A brief description of the contract, used as its title.
    /// Example: "Aquisição de medicamentos corretivos de volémia e outras soluções estéreis"
    pub object_brief_description: String,

    /// The type of the procedure, which is the legal process used to choose the supplier.
    /// Example: "Concurso público", "Ajuste Direto Regime Geral"
    pub contracting_procedure_type: String,

    /// The entities responsible for contracting.
    pub contracting: Option<Vec<BasePortalEntity>>,

    /// The entities contracted.
    pub contracted: Option<Vec<BasePortalEntity>>,

    /// The CPV codes, which identify what is being contracted, separated by `" | "`.
    /// Example: "33600000-6", "44512000-2 | 42715000-1"
    pub cpvs: String,

    /// The designations of the CPV codes, separated by `" | "` in the same order.
    /// Example: "Produtos farmacêuticos"
    pub cpvs_designation: String,

    /// The type of each CPV code, separated by `" | "` in the same order.
    /// Example: "Principal", "Suplementar | Principal"
    pub cpvs_type: String,

    /// The value of each CPV code, separated by `" | "` in the same order.
    /// Example: "61428.54 € ", "4585.0 €  | 59790.0 € "
    pub cpvs_value: String,

    /// The date when the contract was signed, as `dd-mm-yyyy`.
    /// Example: "01-09-2022"
    pub signing_date: Option<String>,

    /// The date when the contract was published, as `dd-mm-yyyy`.
    /// Example: "13-10-2022"
    pub publication_date: String,

    /// The date when the contract was reported as finished, as `dd-mm-yyyy`.
    /// Example: "23-10-2017"
    pub close_date: Option<String>,

    /// The initial contractual price.
    /// Example: "61.428,54 €"
    pub initial_contractual_price: String,

    /// The amount set for the procedure before a supplier is chosen.
    /// It is normally the most the entity will pay.
    /// Example: "7.000,00 €"
    pub estimated_base_price: Option<String>,

    /// The price of the contract when it was closed.
    /// Example: "4.500,00 €"
    pub total_effective_price: Option<String>,

    /// Whether the price is received by the public entity, instead of paid.
    /// Example: false
    pub income: bool,

    /// The legal regime of the contract.
    /// Example: "Código dos Contratos Públicos (DL 18/2008)"
    pub regime: Option<String>,

    /// Set while a published contract is under review, which the portal shows as a warning.
    /// "Em Rectificação" means it is being corrected, "Em Anulação" that it is being cancelled.
    /// Example: "Em Rectificação", "Em Anulação"
    pub contract_status: Option<String>,

    /// The legal reasons for not signing a formal contract document, which is required by default.
    /// For example a low price, urgency, or a purchase under a public supply contract.
    /// Example: "Artigo 95.º, n.º 1, b), locação ou de aquisição de bens móveis ou aquisição de serviços ao abrigo de um contrato público de aprovisionamento"
    pub non_written_contract_justification_types: String,

    /// The types of the contract.
    /// Example: "Aquisição de serviços", "Empreitadas de obras públicas"
    pub contract_types: String,

    /// Whether it is a concession contract, between a grantor and a concessionaire.
    /// Example: false
    #[serde(rename = "contractTypeCS")]
    pub contract_type_cs: bool,

    /// The execution deadline.
    /// Example: "182 dias"
    pub execution_deadline: String,

    /// The places where the contract will be executed, separated by `"<BR/>"`.
    /// Example: "Portugal, Lisboa, Lisboa", "Portugal, Braga, Barcelos<BR/>Portugal, Braga, Barcelos"
    pub execution_place: String,

    /// The legal basis of the contract.
    /// Example: "Artigo 20.º, n.º 1, alínea a) do Código dos Contratos Públicos"
    pub contract_fundamentation_type: String,

    /// The entities that competed for the contract.
    pub contestants: Vec<BasePortalEntity>,

    /// The entities invited to compete for the contract.
    pub invitees: Vec<BasePortalEntity>,

    /// The documents related to the contract.
    pub documents: Vec<BasePortalDocument>,

    /// The link to the procedure documents.
    /// Example: "https://www.cm-valedecambra.pt/pages/727"
    pub contracting_procedure_url: Option<String>,

    /// The Portal BASE id of the announcement. `-1` if there is none.
    /// Example: 324698, -1
    pub announcement_id: i64,

    /// The reason for a direct award. `"Não aplicável"` when it doesn't apply.
    /// Example: "ausência de recursos próprios", "Não aplicável"
    pub direct_award_fundamentation_type: String,

    /// Free text notes about the contract.
    /// Example: "n/a"
    pub observations: Option<String>,

    /// Exact meaning unknown.
    /// Example: true
    pub ccp: bool,

    /// How the contract ended.
    /// Example: "Cumprimento integral do contrato"
    pub end_of_contract_type: Option<String>,

    /// What caused the deadline to change.
    /// Example: "Não Houve"
    pub causes_deadline_change: Option<String>,

    /// What caused the price to change.
    /// Example: "Não Houve"
    pub causes_price_change: Option<String>,

    /// The special measures that apply to the contract.
    /// Example: "Plano de Recuperação e Resiliência (PRR) – artigo 6º da Lei n.º30/2021"
    pub special_measures: Option<String>,

    /// The id of the procedure that created the framework agreement this contract was made under.
    /// A framework agreement sets the terms in advance for the contracts made under it later.
    /// `"Não aplicável."` when there is none.
    /// Example: "181482", "Não aplicável."
    pub framework_agreement_procedure_id: String,

    /// The description of the framework agreement this contract was made under.
    /// `"Não aplicável."` when there is none.
    /// Example: "CP 2015/6 - Acordo quadro para o fornecimento de Medicamentos do Foro Oncológico"
    pub framework_agreement_procedure_description: String,

    /// Whether one procedure covered the needs of several entities, instead of each running its own.
    /// Example: false
    pub centralized_procedure: Option<bool>,

    /// Whether the contract has environmental requirements, shown as "Contrato sustentável".
    /// Example: false
    pub ambient_criteria: bool,

    /// Whether the procedure was chosen for a legal reason other than the contract's value.
    /// For example urgency, or only one supplier being able to do the work.
    /// Example: true
    pub material_criteria: bool,

    /// Exact meaning unknown.
    /// Example: false
    #[serde(rename = "aquisitionStateMemberUE")]
    pub aquisition_state_member_ue: bool,

    /// Exact meaning unknown.
    #[serde(rename = "infoAquisitionStateMemberUE")]
    pub info_aquisition_state_member_ue: Option<serde_json::Value>,

    /// Exact meaning unknown.
    /// Example: false
    pub cocontratantes: bool,

    /// Exact meaning unknown.
    pub group_members: Option<serde_json::Value>,

    /// Whether the contract has modifications.
    /// Example: true
    pub increments: bool,

    /// Whether the entity reported an occurrence, which is a notable event in the contract.
    /// For example a breach of contract, a penalty, or a winner failing to submit its documents.
    /// Example: false
    pub occurrences: bool,

    /// `false` for simplified direct awards, whose contract date is in `close_date`.
    /// Example: true
    pub normal: bool,
}

#[derive(Debug, Deserialize)]
pub struct BasePortalEntity {
    /// The Portal BASE id of the entity.
    /// Example: 165162
    pub id: u64,

    /// The NIF of the entity. `"-"` if it has none.
    /// Example: "503156035", "-"
    pub nif: String,

    /// The name of the entity.
    /// Example: "MRG - Construction, SA."
    pub description: String,
}

#[derive(Debug, Deserialize)]
pub struct BasePortalDocument {
    /// The Portal BASE id of the document.
    /// Example: 313236
    pub id: u64,

    /// The file name of the document.
    /// Example: "contrato numerado e assinado.pdf"
    pub description: String,
}
