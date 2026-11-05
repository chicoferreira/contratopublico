//! Client for the official BASE API at <https://www.base.gov.pt/APIBase2V2>, which needs a token.
//! Not to be confused with [`crate::sources::base_portal`], which scrapes the public Portal BASE
//! website.

pub mod client;
pub mod scraper;

use crate::sources::text::Text;
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

    /// The announcement number. Empty if there was no announcement.
    /// Example: "3539/2024", "21438/2023"
    #[serde(rename = "nAnuncio")]
    pub announcement_number: Text,

    /// The type of announcement. Empty if there was no announcement.
    /// Example: "Anúncio de procedimento", "Anúncio de concurso urgente"
    #[serde(rename = "TipoAnuncio")]
    pub announcement_type: Text,

    /// The announcement id in Diário da República. `"-1"` or empty if there is none.
    /// Example: "417311274"
    #[serde(rename = "idINCM")]
    pub incm_id: Text,

    /// The types of the contract.
    /// Example: "Aquisição de serviços", "Empreitadas de obras públicas"
    #[serde(rename = "tipoContrato")]
    pub contract_types: Option<Vec<Text>>,

    /// The id of the procedure that originated the contract.
    /// One procedure can originate several contracts.
    /// Example: "6938074"
    #[serde(rename = "idprocedimento")]
    pub procedure_id: Text,

    /// The type of the procedure, which is the legal process used to choose the supplier.
    /// Example: "Ajuste Direto Regime Geral", "Concurso público"
    #[serde(rename = "tipoprocedimento")]
    pub procedure_type: Text,

    /// A brief description of the contract, used as its title.
    /// Example: "Programa Cheques Psicólogo/Nutricionista"
    #[serde(rename = "objectoContrato")]
    pub object_brief_description: Text,

    /// The detailed description of the contract.
    /// Example: "Aquisição de serviço de viagens e alojamento – Plataforma SIAV"
    #[serde(rename = "descContrato")]
    pub description: Text,

    /// The entities responsible for contracting.
    #[serde(rename = "adjudicante")]
    pub contracting: Option<Vec<BaseApiEntity>>,

    /// The entities contracted.
    #[serde(rename = "adjudicatarios")]
    pub contracted: Option<Vec<BaseApiEntity>>,

    /// The date when the contract was published, as `yyyy-mm-dd`.
    /// Example: "2024-12-27"
    #[serde(rename = "dataPublicacao")]
    pub publication_date: Text,

    /// The date when the contract was signed, as `yyyy-mm-dd`. Empty when unset.
    /// It is when the contract comes into existence, concluded by both the entity and the supplier.
    /// It comes after the award decision, once the supplier submits its qualification documents.
    /// Some procedures also have a waiting period before it, for challenges to the award decision.
    /// Example: "2024-12-03"
    #[serde(rename = "dataCelebracaoContrato")]
    pub signing_date: Text,

    /// The initial contractual price, in euros.
    /// Example: 15000.0
    #[serde(rename = "precoContratual")]
    pub initial_contractual_price: f64,

    /// The CPV codes, which identify what is being contracted.
    #[serde(rename = "CPV")]
    pub cpvs: Option<Vec<BaseApiCpv>>,

    /// The execution deadline, in days.
    /// Example: 365
    #[serde(rename = "prazoExecucao")]
    pub execution_deadline_days: i32,

    /// The places where the contract will be executed.
    /// Example: "Portugal, Lisboa, Oeiras", "Portugal"
    #[serde(rename = "localExecucao")]
    pub execution_places: Option<Vec<Text>>,

    /// The legal basis of the procedure.
    /// Example: "Artigo 20.º, n.º 1, alínea d) do Código dos Contratos Públicos"
    #[serde(rename = "fundamentacao")]
    pub contract_fundamentation_type: Text,

    /// Whether one procedure covered the needs of several entities, instead of each running its own.
    /// Example: "Sim", "Não"
    #[serde(rename = "ProcedimentoCentralizado")]
    pub centralized_procedure: Text,

    /// The id of the procedure that created the framework agreement this contract was made under.
    /// A framework agreement sets the terms in advance for the contracts made under it later.
    /// Empty when there is none.
    /// Example: "5715196"
    #[serde(rename = "numAcordoQuadro")]
    pub framework_agreement_id: Text,

    /// The description of the framework agreement this contract was made under.
    /// Empty when there is none.
    /// Example: "Acordo Quadro de Viagens e Alojamento"
    #[serde(rename = "DescrAcordoQuadro")]
    pub framework_agreement_description: Text,

    /// The amount set for the procedure before a supplier is chosen, in euros. `0.0` when unset.
    /// It is normally the most the entity will pay.
    /// Example: 20000.0
    #[serde(rename = "precoBaseProcedimento")]
    pub estimated_base_price: f64,

    /// The date of the award decision, as `yyyy-mm-dd`. Empty when unset.
    /// It is when the entity alone decides which supplier wins the contract, before the signing.
    /// If the chosen supplier then fails to submit its documents, the next one gets the contract.
    /// Example: "2024-02-29"
    #[serde(rename = "dataDecisaoAdjudicacao")]
    pub award_decision_date: Text,

    /// The date when the contract was reported as finished, as `yyyy-mm-dd`. Empty when unset.
    /// Example: "2024-12-31"
    #[serde(rename = "dataFechoContrato")]
    pub close_date: Text,

    /// The amount actually paid when the contract was closed, in euros. `0.0` when unset.
    /// Example: 7500.0
    #[serde(rename = "PrecoTotalEfetivo")]
    pub total_effective_price: f64,

    /// The legal regime of the contract.
    /// Example: "Código dos Contratos Públicos ( DL 111-B/2017 )", "DLR n.º 27/2015/A, de 29.12"
    #[serde(rename = "regime")]
    pub regime: Text,

    /// The legal reasons for not signing a formal contract document, which is required by default.
    /// For example a low price, urgency, or a purchase under a public supply contract.
    /// Example: "Artigo 5.º - Contratação excluída “geral”"
    #[serde(rename = "justifNReducEscrContrato")]
    pub non_written_contract_justification_types: Option<Vec<Text>>,

    /// How the contract ended. Empty when unset.
    /// Example: "Resolução do Contrato", "Revogação do Contrato"
    #[serde(rename = "tipoFimContrato")]
    pub end_of_contract_type: Text,

    /// Whether the procedure was chosen for a legal reason other than the contract's value.
    /// For example urgency, or only one supplier being able to do the work.
    /// Example: "Sim", "Não"
    #[serde(rename = "CritMateriais")]
    pub material_criteria: Text,

    /// The entities that competed for the contract, as NIF and name joined by `-`.
    /// The NIF is `"-"` for foreign entities and natural persons, hidden for data protection.
    /// Example: "502248793-MYLAN, LDA.", "--Iberdrola Generation, S.A.U"
    #[serde(rename = "concorrentes")]
    pub contestants: Option<Vec<Text>>,

    /// The link to the procedure documents. Empty when unset.
    /// Example: "https://community.vortal.biz"
    #[serde(rename = "linkPecasProc")]
    pub contracting_procedure_url: Text,

    /// Free text notes about the contract.
    /// Example: "N/A"
    #[serde(rename = "Observacoes")]
    pub observations: Text,

    /// Whether the contract has environmental requirements, shown as "Contrato sustentável".
    /// Example: "Sim", "Não"
    #[serde(rename = "ContratEcologico")]
    pub ambient_criteria: Text,

    /// The year the API files the contract under, which `fetch_by_year` filters on.
    /// Example: 2024
    #[serde(rename = "Ano")]
    pub year: i32,

    /// The reason for a direct award. Empty when it doesn't apply.
    /// Example: "ausência de recursos próprios", "outros"
    #[serde(rename = "fundamentAjusteDireto")]
    pub direct_award_fundamentation_type: Text,

    /// The NUTS regions where the contract will be executed.
    /// Example: "PT150 - Algarve"
    #[serde(rename = "NUTs")]
    pub nuts: Option<Vec<Text>>,

    /// The lots of the procedure, one row per lot and award criterion.
    /// A procedure with no lots has a single lot `"0"`, which holds its award criterion.
    #[serde(rename = "Lotes")]
    pub lots: Option<Vec<BaseApiLot>>,

    /// Whether the contract came from a procedure that several entities ran together.
    /// Each entity usually signs its own contract, so this contract often has only one of them.
    /// Example: "Sim", "Não"
    #[serde(rename = "compraConjunta")]
    pub joint_purchase: Text,

    /// The special procurement measure the contract was made under. Empty when there is none.
    /// Example: "Setor da saúde e do apoio social – artigo 5º da Lei n.º30/2021"
    #[serde(rename = "TipoMEC")]
    pub special_measures: Text,

    /// The entities invited to submit a proposal.
    #[serde(rename = "entidadesConvidadas")]
    pub invitees: Option<Vec<BaseApiInvitee>>,

    /// The reasons for changing the contract deadline. Empty when unset.
    /// Example: "Não aplicável"
    #[serde(rename = "CausasAlteraPrazo")]
    pub causes_deadline_change: Text,

    /// The reasons for changing the contract price. Empty when unset.
    /// Example: "Terminou a vigência do contrato"
    #[serde(rename = "CausasAlteraPreco")]
    pub causes_price_change: Text,

    /// The announcement id, which is the same in the API and the portal. Empty when there is none.
    /// Example: "374564"
    #[serde(rename = "IDAnuncio")]
    pub announcement_id: Text,

    /// Set while a published contract is under review, which the portal shows as a warning.
    /// "Em Rectificação" means it is being corrected, "Em Anulação" that it is being cancelled.
    /// Empty otherwise.
    /// Example: "Em Rectificação", "Em Anulação"
    #[serde(rename = "estadoContrato")]
    pub contract_status: Text,

    /// The estimated value of the whole procedure, in euros. `0.0` when unset.
    /// It is set before the procedure starts, and covers every lot, renewal and option.
    /// It is never below the base price, and is higher when the procedure has other contracts too.
    /// Example: 1121551.0
    #[serde(rename = "precoEstimadoProc")]
    pub estimated_procedure_price: f64,

    /// Whether the entity reported an occurrence, which is a notable event in the contract.
    /// For example a breach of contract, a penalty, or a winner failing to submit its documents.
    /// Example: false
    #[serde(rename = "ocorrencias")]
    pub occurrences: bool,
}

/// A contracting or contracted entity of a [`BaseApiContract`].
#[derive(Debug, Deserialize, Serialize)]
pub struct BaseApiEntity {
    /// The NIF of the entity.
    /// `"-"` for foreign entities and natural persons, whose NIF is hidden for data protection.
    /// Example: "508142156", "-"
    #[serde(rename = "nifEntContrato")]
    pub nif: Text,

    /// The name of the entity.
    /// Example: "Unidade Local de Saúde de Gaia/Espinho, E. P. E."
    #[serde(rename = "descEntContrato")]
    pub description: Option<Text>,

    /// The ISO 3166-1 alpha-3 code of the entity's country.
    /// Example: "PRT", "ESP"
    #[serde(rename = "codPaisEntContrato")]
    pub country_code: Option<Text>,

    /// The NUTS code of the entity's location.
    /// Example: "PT1A0"
    #[serde(rename = "NUTsCod")]
    pub nuts_code: Option<Text>,

    /// The name of the NUTS region of the entity's location.
    /// Example: "Grande Lisboa"
    #[serde(rename = "NUTsDesc")]
    pub nuts_description: Option<Text>,

    /// Whether the entity is an PME (Pequena e Média Empresa).
    /// Only set for contracted entities, and only when it is one.
    /// Example: "Sim"
    #[serde(rename = "adjudicatarioPME")]
    pub sme: Option<Text>,

    /// Example: "Organismo de direito público", "Autoridade local"
    #[serde(rename = "tipoEntidade")]
    pub entity_type: Option<Text>,

    /// The main economic activity of the entity.
    /// Example: "Saúde", "Defesa"
    #[serde(rename = "atividadeEntidade")]
    pub activity: Option<Text>,
}

/// An entity invited to submit a proposal for a [`BaseApiContract`].
#[derive(Debug, Deserialize, Serialize)]
pub struct BaseApiInvitee {
    /// The NIF of the entity.
    /// `"-"` for foreign entities and natural persons. Hidden for data protection.
    /// Example: "506661393", "-"
    #[serde(rename = "NIPCEntidade")]
    pub nif: Text,

    /// The name of the entity.
    /// Example: "Construções Refoiense, Lda."
    #[serde(rename = "Entidade")]
    pub description: Option<Text>,

    /// The ISO 3166-1 alpha-3 code of the entity's country.
    /// Example: "PRT"
    #[serde(rename = "AliasPais")]
    pub country_code: Option<Text>,

    /// The name of the entity's country.
    /// Example: "Portugal"
    #[serde(rename = "Pais")]
    pub country: Option<Text>,
}

/// A CPV code of a [`BaseApiContract`].
#[derive(Debug, Deserialize, Serialize)]
pub struct BaseApiCpv {
    /// The CPV code.
    /// Example: "33600000-6"
    #[serde(rename = "cpvID")]
    pub code: Text,

    /// The designation of the CPV code.
    /// Example: "Produtos farmacêuticos"
    #[serde(rename = "cpvdescricao")]
    pub designation: Text,

    /// The value of the CPV code, in euros. `0.0` when unset.
    /// Example: 6758.0
    #[serde(rename = "cpvValor")]
    pub value: f64,

    /// The type of the CPV code.
    /// Example: "Principal", "Complementar"
    #[serde(rename = "CPVObjeto")]
    pub kind: Text,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct BaseApiLot {
    /// The award criterion of the lot.
    /// Example: "Preço", "Fator Preço"
    #[serde(rename = "criterioLote")]
    pub criterion: Option<Text>,

    /// The weight of the award criterion, whose unit depends on the procedure.
    /// Example: 65.0
    #[serde(rename = "criterioAdjudicacaoPonderacao")]
    pub criterion_weight: f64,

    /// The number of the lot in the procedure. `"0"` when the procedure has no lots.
    /// Example: "1", "0"
    #[serde(rename = "numLot")]
    pub number: Option<Text>,

    /// The name of the lot.
    /// Example: "Lote 1"
    #[serde(rename = "designacaoLote")]
    pub designation: Option<Text>,

    /// The detailed description of the lot.
    #[serde(rename = "descricaoLote")]
    pub description: Option<Text>,

    /// The type of the award criterion.
    /// Example: "Monofator", "Multifator"
    #[serde(rename = "criterioAdjudicacaoTipo")]
    pub criterion_type: Option<Text>,

    /// The category of the evaluation factor of the award criterion.
    /// Example: "Preço", "Qualidade", "Outros"
    #[serde(rename = "criterioAdjudicacaoTipoFator")]
    pub criterion_factor_type: Option<Text>,

    /// How the award criterion is weighted.
    /// Example: "Ponderação (percentagem, valor exato)"
    #[serde(rename = "criterioAdjudicacaoFator")]
    pub criterion_factor: Option<Text>,

    /// The number of proposals received for the lot.
    /// Example: 7
    #[serde(rename = "NumPropostas")]
    pub proposals: u32,
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
    pub fundamentation: Text,

    /// The type of act that made the modification.
    /// Example: "Acordo entre as partes", "Acto Administrativo"
    #[serde(rename = "modifContratoTipoAto")]
    pub act_type: Text,

    /// The date of the modification, as `dd/mm/yyyy 00:00:00`.
    /// Example: "30/12/2024 00:00:00"
    #[serde(rename = "modifContratoData")]
    pub date: Text,

    /// The contract price after the modification, in euros.
    /// Example: 24950.66
    #[serde(rename = "modifContratoPrecoAlterado")]
    pub changed_price: f64,

    /// The execution deadline in days, as a string.
    /// Example: "365"
    #[serde(rename = "modifPrazoExecucao")]
    pub execution_deadline_days: Text,

    /// The date when the modification was published, as `dd/mm/yyyy 00:00:00`.
    /// Example: "19/12/2024 00:00:00"
    #[serde(rename = "modifDataPublicacao")]
    pub publication_date: Text,

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
