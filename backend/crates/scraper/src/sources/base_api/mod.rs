//! Client for the official BASE API at <https://www.base.gov.pt/APIBase2>, which needs a token.
//! Not to be confused with [`crate::sources::base_portal`], which scrapes the public Portal BASE
//! website.

pub mod client;
pub mod scraper;

use serde::{Deserialize, Deserializer, Serialize, Serializer};

/// A contract row from the BASE API.
#[derive(Debug, Deserialize, Serialize)]
pub struct BaseApiContract {
    /// The contract id, which is the same in the API and the portal.
    /// Example: 10804349
    #[serde(
        rename = "idcontrato",
        deserialize_with = "deserialize_u64",
        serialize_with = "serialize_u64"
    )]
    pub id: u64,

    /// The id of the procedure that originated the contract.
    /// One procedure can originate several contracts.
    /// Example: "6938074"
    #[serde(rename = "idprocedimento")]
    pub procedure_id: String,

    /// The announcement number. Empty if there was no announcement.
    /// Example: "3539/2024", "21438/2023"
    #[serde(rename = "nAnuncio")]
    pub announcement_number: String,

    /// The type of announcement. Empty if there was no announcement.
    /// Example: "Anúncio de procedimento", "Anúncio de concurso urgente"
    #[serde(rename = "TipoAnuncio")]
    pub announcement_type: String,

    /// The announcement id in Diário da República. `"-1"` or empty if there is none.
    /// Example: "417311274"
    #[serde(rename = "idINCM")]
    pub incm_id: String,

    /// The types of the contract.
    /// Example: "Aquisição de serviços", "Empreitadas de obras públicas"
    #[serde(rename = "tipoContrato")]
    pub contract_types: Option<Vec<String>>,

    /// The type of the procedure, which is the legal process used to choose the supplier.
    /// Example: "Ajuste Direto Regime Geral", "Concurso público"
    #[serde(rename = "tipoprocedimento")]
    pub procedure_type: String,

    /// A brief description of the contract, used as its title.
    /// Example: "Programa Cheques Psicólogo/Nutricionista"
    #[serde(rename = "objectoContrato")]
    pub object_brief_description: String,

    /// The description of the contract.
    /// Example: "Aquisição de serviço de viagens e alojamento – Plataforma SIAV"
    #[serde(rename = "descContrato")]
    pub description: String,

    /// The entities responsible for contracting.
    /// Example: "508142156 - Unidade Local de Saúde de Gaia/Espinho, E. P. E."
    #[serde(rename = "adjudicante")]
    pub contracting: Option<Vec<String>>,

    /// The entities contracted. Entities with no NIF start with `"- - "`.
    /// Example: "500162166 - LAB. PFIZER, LDA.", "- - Quint Wellington Redwood Iberia SL"
    #[serde(rename = "adjudicatarios")]
    pub contracted: Option<Vec<String>>,

    /// The entities that competed for the contract, with no spaces around the first dash.
    /// Example: "502248793-MYLAN, LDA."
    #[serde(rename = "concorrentes")]
    pub contestants: Option<Vec<String>>,

    /// The NIFs of the contracted entities that are SMEs.
    /// Example: "506019608", "-"
    #[serde(rename = "adjudicatarioPMEs")]
    pub contracted_smes: Option<Vec<String>>,

    /// The date when the contract was published, as `dd/mm/yyyy`.
    /// Example: "27/12/2024"
    #[serde(rename = "dataPublicacao")]
    pub publication_date: String,

    /// The date when the contract was signed, as `dd/mm/yyyy`. Empty when unset.
    /// Example: "03/12/2024"
    #[serde(rename = "dataCelebracaoContrato")]
    pub signing_date: String,

    /// The date of the award decision, as `dd/mm/yyyy`. Empty when unset.
    /// Example: "29/02/2024"
    #[serde(rename = "dataDecisaoAdjudicacao")]
    pub award_decision_date: String,

    /// The date when the contract was reported as finished, as `dd/mm/yyyy`. Empty when unset.
    /// Example: "31/12/2024"
    #[serde(rename = "dataFechoContrato")]
    pub close_date: String,

    /// The initial contractual price, in euros.
    /// Example: 15000.0
    #[serde(rename = "precoContratual")]
    pub initial_contractual_price: f64,

    /// The amount set for the procedure before a supplier is chosen, in euros. `0.0` when unset.
    /// It is normally the most the entity will pay.
    /// Example: 20000.0
    #[serde(rename = "precoBaseProcedimento")]
    pub estimated_base_price: f64,

    /// The price of the contract when it was closed, in euros. `0.0` when unset.
    /// Example: 7500.0
    #[serde(rename = "PrecoTotalEfetivo")]
    pub total_effective_price: f64,

    /// The CPV codes, which identify what is being contracted.
    /// Example: "33600000-6 - Produtos farmacêuticos"
    #[serde(rename = "cpv")]
    pub cpvs: Option<Vec<String>>,

    /// The execution deadline, in days.
    /// Example: 365
    #[serde(rename = "prazoExecucao")]
    pub execution_deadline_days: i32,

    /// The places where the contract will be executed.
    /// Example: "Portugal, Lisboa, Oeiras", "Portugal"
    #[serde(rename = "localExecucao")]
    pub execution_places: Option<Vec<String>>,

    /// The NUTS regions of the contract.
    /// Example: "PT150 - Algarve"
    #[serde(rename = "NUTs")]
    pub nuts: Option<Vec<String>>,

    /// The lots of the procedure, which are free text.
    /// Example: "Lote 1", "MEDICAMENTOS"
    #[serde(rename = "Lotes")]
    pub lots: Option<Vec<String>>,

    /// The legal basis of the contract.
    /// Example: "Artigo 20.º, n.º 1, alínea d) do Código dos Contratos Públicos"
    #[serde(rename = "fundamentacao")]
    pub contract_fundamentation_type: String,

    /// The reason for a direct award. Empty when it doesn't apply.
    /// Example: "ausência de recursos próprios", "outros"
    #[serde(rename = "fundamentAjusteDireto")]
    pub direct_award_fundamentation_type: String,

    /// The legal reasons for not signing a formal contract document, which is required by default.
    /// For example a low price, urgency, or a purchase under a public supply contract.
    /// Example: "Artigo 5.º - Contratação excluída “geral”"
    #[serde(rename = "justifNReducEscrContrato")]
    pub non_written_contract_justification_types: Option<Vec<String>>,

    /// Whether one procedure covered the needs of several entities, instead of each running its own.
    /// Example: "Sim", "Não"
    #[serde(rename = "ProcedimentoCentralizado")]
    pub centralized_procedure: String,

    /// The id of the procedure that created the framework agreement this contract was made under.
    /// A framework agreement sets the terms in advance for the contracts made under it later.
    /// Empty when there is none.
    /// Example: "5715196"
    #[serde(rename = "numAcordoQuadro")]
    pub framework_agreement_id: String,

    /// The description of the framework agreement this contract was made under.
    /// Empty when there is none.
    /// Example: "Acordo Quadro de Viagens e Alojamento"
    #[serde(rename = "DescrAcordoQuadro")]
    pub framework_agreement_description: String,

    /// The legal regime of the contract.
    /// Example: "Código dos Contratos Públicos ( DL 111-B/2017 )", "DLR n.º 27/2015/A, de 29.12"
    #[serde(rename = "regime")]
    pub regime: String,

    /// How the contract ended. Empty when unset.
    /// Example: "Resolução do Contrato", "Revogação do Contrato"
    #[serde(rename = "tipoFimContrato")]
    pub end_of_contract_type: String,

    /// Whether the procedure was chosen for a legal reason other than the contract's value.
    /// For example urgency, or only one supplier being able to do the work.
    /// Example: "Sim", "Não"
    #[serde(rename = "CritMateriais")]
    pub material_criteria: String,

    /// Whether the contract has environmental requirements, shown as "Contrato sustentável".
    /// Example: "Sim", "Não"
    #[serde(rename = "ContratEcologico")]
    pub ambient_criteria: String,

    /// The criterion used to award the contract. Empty when unset.
    /// Example: "Monofator", "Melhor relação qualidade – preço"
    #[serde(rename = "TipoCriterioAdjudicacao")]
    pub award_criterion: String,

    /// The link to the procedure documents. Empty when unset.
    /// Example: "https://community.vortal.biz"
    #[serde(rename = "linkPecasProc")]
    pub contracting_procedure_url: String,

    /// Free text notes about the contract.
    /// Example: "N/A"
    #[serde(rename = "Observacoes")]
    pub observations: String,

    /// The year the API files the contract under, which `fetch_by_year` filters on.
    /// Example: 2024
    #[serde(rename = "Ano")]
    pub year: i32,
}

/// A modification to a contract from the BASE API.
///
/// A contract can have several.
#[derive(Debug, Deserialize)]
pub struct BaseApiModification {
    /// The id of the modified contract.
    /// Example: 10424433
    #[serde(rename = "idcontrato", deserialize_with = "deserialize_u64")]
    pub contract_id: u64,

    /// The legal reason for the modification.
    /// Example: "Trabalhos Complementares (art. 370.º do CCP)"
    #[serde(rename = "modifContratoFundamento")]
    pub fundamentation: String,

    /// The type of act that made the modification.
    /// Example: "Acordo entre as partes", "Acto Administrativo"
    #[serde(rename = "modifContratoTipoAto")]
    pub act_type: String,

    /// The date of the modification, as `dd/mm/yyyy 00:00:00`.
    /// Example: "30/12/2024 00:00:00"
    #[serde(rename = "modifContratoData")]
    pub date: String,

    /// The date when the modification was published, as `dd/mm/yyyy 00:00:00`.
    /// Example: "19/12/2024 00:00:00"
    #[serde(rename = "modifDataPublicacao")]
    pub publication_date: String,

    /// The contract price after the modification, in euros.
    /// Example: 24950.66
    #[serde(rename = "modifContratoPrecoAlterado")]
    pub changed_price: f64,

    /// The execution deadline in days, as a string.
    /// Example: "365"
    #[serde(rename = "modifPrazoExecucao")]
    pub execution_deadline_days: String,

    /// The year the API files the modification under, which is the contract's year.
    /// Example: 2024
    #[serde(rename = "Ano")]
    pub year: i32,
}

fn deserialize_u64<'de, D: Deserializer<'de>>(deserializer: D) -> Result<u64, D::Error> {
    String::deserialize(deserializer)?
        .parse()
        .map_err(serde::de::Error::custom)
}

fn serialize_u64<S: Serializer>(value: &u64, serializer: S) -> Result<S::Ok, S::Error> {
    serializer.collect_str(value)
}
