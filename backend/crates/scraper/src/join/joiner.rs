use std::hash::{DefaultHasher, Hash, Hasher};

use anyhow::Context;
use chrono::NaiveDate;
use common::{Contract, ContractType, Cpv, Currency, Document, Entity, ProcedureType};
use itertools::Itertools;

use crate::sources::{
    base_api::{BaseApiContract, BaseApiCpv, BaseApiEntity, BaseApiInvitee},
    base_portal::{BasePortalContract, BasePortalDocument, BasePortalEntity},
    text::Text,
};

const DIRECT_AWARD_NOT_APPLICABLE: &str = "Não aplicável";
const FRAMEWORK_AGREEMENT_NOT_APPLICABLE: &str = "Não aplicável.";

pub fn join_contract(sources: Sources) -> anyhow::Result<Contract> {
    Ok(Contract {
        id: sources.api_first(|c| c.id, |c| c.id),
        procedure_type: sources.try_api_first(
            |c| parse_procedure_type(&c.procedure_type),
            |c| parse_procedure_type(&c.contracting_procedure_type),
        )?,
        publication_date: sources.try_api_first(
            |c| parse_api_date(&c.publication_date),
            |c| parse_portal_date(&c.publication_date),
        )?,
        signing_date: sources.try_api_first_option(
            |c| parse_optional_api_date(&c.signing_date),
            |c| c.signing_date.as_deref().map(parse_portal_date).transpose(),
        )?,
        object_brief_description: sources.api_first(
            |c| c.object_brief_description.to_string(),
            |c| c.object_brief_description.to_string(),
        ),
        initial_contractual_price: sources.try_api_first(
            |c| Ok(euros_to_currency(c.initial_contractual_price)),
            |c| parse_portal_euros(&c.initial_contractual_price),
        )?,
        description: sources.api_first_option(
            |c| c.description.non_empty(),
            |c| c.description.as_deref().map(String::from),
        ),
        contracting: sources.portal_first(
            |c| portal_entities(c.contracting.as_deref().unwrap_or_default()),
            |c| api_entities(&c.contracting),
        ),
        contracted: sources.portal_first(
            |c| portal_entities(c.contracted.as_deref().unwrap_or_default()),
            |c| api_entities(&c.contracted),
        ),
        cpvs: sources.try_api_first(|c| api_cpvs(&c.cpvs), |c| parse_portal_cpvs(&c.cpvs))?,
        regime: sources.api_first_option(
            |c| c.regime.non_empty(),
            |c| c.regime.as_deref().map(String::from),
        ),
        contract_status: sources.api_first_option(
            |c| c.contract_status.non_empty(),
            |c| c.contract_status.as_deref().map(String::from),
        ),
        non_written_contract_justification_types: sources.portal_first(
            |c| parse_portal_list(&c.non_written_contract_justification_types, "<br/>"),
            |c| api_list(&c.non_written_contract_justification_types),
        ),
        contract_types: sources.try_api_first(
            |c| parse_contract_types(api_list(&c.contract_types)),
            |c| parse_contract_types(parse_portal_list(&c.contract_types, "<br/>")),
        )?,
        execution_deadline_days: sources.try_api_first(
            |c| parse_api_execution_deadline(c.execution_deadline_days),
            |c| parse_portal_execution_deadline(&c.execution_deadline),
        )?,
        execution_places: sources.api_first(
            |c| api_list(&c.execution_places),
            |c| parse_portal_list(&c.execution_place, "<BR/>"),
        ),
        contract_fundamentation_type: sources.api_first(
            |c| c.contract_fundamentation_type.to_string(),
            |c| c.contract_fundamentation_type.to_string(),
        ),
        contestants: sources.portal_first(
            |c| portal_entities(&c.contestants),
            |c| api_contestants(&c.contestants),
        ),
        invitees: sources.portal_first(
            |c| portal_entities(&c.invitees),
            |c| api_invitees(&c.invitees),
        ),
        documents: sources.portal_or_default(|c| portal_documents(&c.documents)),
        contracting_procedure_url: sources.api_first_option(
            |c| c.contracting_procedure_url.non_empty(),
            |c| c.contracting_procedure_url.as_deref().map(String::from),
        ),
        announcement_id: sources.try_api_first_option(
            |c| parse_optional_api_id(&c.announcement_id),
            |c| Ok(usize::try_from(c.announcement_id).ok()),
        )?,
        direct_award_fundamentation_type: sources.portal_first_option(
            |c| {
                Some(c.direct_award_fundamentation_type.to_string())
                    .filter(|value| value != DIRECT_AWARD_NOT_APPLICABLE)
            },
            |c| c.direct_award_fundamentation_type.non_empty(),
        ),
        observations: sources.api_first_option(
            |c| c.observations.non_empty(),
            |c| c.observations.as_deref().map(String::from),
        ),
        end_of_contract_type: sources.portal_first_option(
            |c| c.end_of_contract_type.as_deref().map(String::from),
            |c| c.end_of_contract_type.non_empty(),
        ),
        close_date: sources.try_api_first_option(
            |c| parse_optional_api_date(&c.close_date),
            |c| c.close_date.as_deref().map(parse_portal_date).transpose(),
        )?,
        total_effective_price: sources.try_api_first_option(
            |c| Ok(optional_currency(c.total_effective_price)),
            |c| {
                c.total_effective_price
                    .as_deref()
                    .map(parse_portal_euros)
                    .transpose()
            },
        )?,
        causes_deadline_change: sources.api_first_option(
            |c| c.causes_deadline_change.non_empty(),
            |c| c.causes_deadline_change.as_deref().map(String::from),
        ),
        causes_price_change: sources.api_first_option(
            |c| c.causes_price_change.non_empty(),
            |c| c.causes_price_change.as_deref().map(String::from),
        ),
        estimated_base_price: sources.try_api_first_option(
            |c| {
                Ok((c.estimated_base_price > 0.0)
                    .then(|| euros_to_currency(c.estimated_base_price)))
            },
            |c| {
                Ok(c.estimated_base_price
                    .as_deref()
                    .map(parse_portal_euros)
                    .transpose()?
                    .filter(|price| price.0 > 0))
            },
        )?,
        award_decision_date: sources.try_api_first_option(
            |c| parse_optional_api_date(&c.award_decision_date),
            |_| Ok(None),
        )?,
        framework_agreement_description: sources.api_first_option(
            |c| c.framework_agreement_description.non_empty(),
            |c| {
                c.framework_agreement_procedure_description
                    .non_empty()
                    .filter(|description| description != FRAMEWORK_AGREEMENT_NOT_APPLICABLE)
            },
        ),
        special_measures: sources.api_first_option(
            |c| c.special_measures.non_empty(),
            |c| c.special_measures.as_ref().and_then(Text::non_empty),
        ),
        ambient_criteria: sources
            .api_first(|c| &c.ambient_criteria == "Sim", |c| c.ambient_criteria),
    })
}

pub enum Sources<'a> {
    Api(&'a BaseApiContract),
    Portal(&'a BasePortalContract),
    Both(&'a BaseApiContract, &'a BasePortalContract),
}

impl<'a> Sources<'a> {
    /// Returns `None` if neither source has the contract.
    pub fn new(
        api: &'a Option<BaseApiContract>,
        portal: &'a Option<BasePortalContract>,
    ) -> Option<Self> {
        match (api, portal) {
            (Some(api), Some(portal)) => Some(Self::Both(api, portal)),
            (Some(api), None) => Some(Self::Api(api)),
            (None, Some(portal)) => Some(Self::Portal(portal)),
            (None, None) => None,
        }
    }

    fn api_first<T>(
        &self,
        api: impl FnOnce(&BaseApiContract) -> T,
        portal: impl FnOnce(&BasePortalContract) -> T,
    ) -> T {
        match self {
            Self::Api(contract) | Self::Both(contract, _) => api(contract),
            Self::Portal(contract) => portal(contract),
        }
    }

    fn portal_first<T>(
        &self,
        portal: impl FnOnce(&BasePortalContract) -> T,
        api: impl FnOnce(&BaseApiContract) -> T,
    ) -> T {
        match self {
            Self::Portal(contract) | Self::Both(_, contract) => portal(contract),
            Self::Api(contract) => api(contract),
        }
    }

    fn try_api_first<T>(
        &self,
        api: impl FnOnce(&BaseApiContract) -> anyhow::Result<T>,
        portal: impl FnOnce(&BasePortalContract) -> anyhow::Result<T>,
    ) -> anyhow::Result<T> {
        match self {
            Self::Api(contract) => api(contract),
            Self::Portal(contract) => portal(contract),
            Self::Both(api_contract, portal_contract) => {
                let value = api(api_contract)?;
                portal(portal_contract)?;

                Ok(value)
            }
        }
    }

    fn try_api_first_option<T>(
        &self,
        api: impl FnOnce(&BaseApiContract) -> anyhow::Result<Option<T>>,
        portal: impl FnOnce(&BasePortalContract) -> anyhow::Result<Option<T>>,
    ) -> anyhow::Result<Option<T>> {
        match self {
            Self::Api(contract) => api(contract),
            Self::Portal(contract) => portal(contract),
            Self::Both(api_contract, portal_contract) => {
                let api_value = api(api_contract)?;
                let portal_value = portal(portal_contract)?;

                Ok(api_value.or(portal_value))
            }
        }
    }

    fn api_first_option<T>(
        &self,
        api: impl FnOnce(&BaseApiContract) -> Option<T>,
        portal: impl FnOnce(&BasePortalContract) -> Option<T>,
    ) -> Option<T> {
        match self {
            Self::Api(contract) => api(contract),
            Self::Portal(contract) => portal(contract),
            Self::Both(api_contract, portal_contract) => {
                api(api_contract).or_else(|| portal(portal_contract))
            }
        }
    }

    fn portal_first_option<T>(
        &self,
        portal: impl FnOnce(&BasePortalContract) -> Option<T>,
        api: impl FnOnce(&BaseApiContract) -> Option<T>,
    ) -> Option<T> {
        match self {
            Self::Api(contract) => api(contract),
            Self::Portal(contract) => portal(contract),
            Self::Both(api_contract, portal_contract) => {
                portal(portal_contract).or_else(|| api(api_contract))
            }
        }
    }

    fn portal_or_default<T: Default>(&self, portal: impl FnOnce(&BasePortalContract) -> T) -> T {
        match self {
            Self::Portal(contract) | Self::Both(_, contract) => portal(contract),
            Self::Api(_) => T::default(),
        }
    }
}

fn parse_api_date(date: &str) -> anyhow::Result<NaiveDate> {
    NaiveDate::parse_from_str(date, "%Y-%m-%d").with_context(|| format!("Invalid date: {date}"))
}

fn parse_optional_api_date(date: &str) -> anyhow::Result<Option<NaiveDate>> {
    (!date.is_empty()).then(|| parse_api_date(date)).transpose()
}

fn parse_optional_api_id(id: &str) -> anyhow::Result<Option<usize>> {
    (!id.is_empty())
        .then(|| id.parse().with_context(|| format!("Invalid id: {id}")))
        .transpose()
}

fn parse_api_execution_deadline(days: i32) -> anyhow::Result<usize> {
    usize::try_from(days).with_context(|| format!("Invalid execution deadline: {days}"))
}

fn euros_to_currency(euros: f64) -> Currency {
    Currency((euros * 100.0).round() as isize)
}

fn optional_currency(euros: f64) -> Option<Currency> {
    (euros != 0.0).then(|| euros_to_currency(euros))
}

fn api_cpvs(cpvs: &Option<Vec<BaseApiCpv>>) -> anyhow::Result<Vec<Cpv>> {
    cpvs.iter()
        .flatten()
        .unique_by(|cpv| &cpv.code)
        .map(|cpv| parse_cpv(&cpv.code))
        .collect()
}

fn parse_cpv(code: &str) -> anyhow::Result<Cpv> {
    let code = code.trim();
    Cpv::from_code(code).ok_or_else(|| anyhow::anyhow!("Unknown CPV code: {code}"))
}

fn api_list(list: &Option<Vec<Text>>) -> Vec<String> {
    list.iter()
        .flatten()
        .map(|item| item.trim().to_string())
        .filter(|item| !item.is_empty())
        .unique()
        .collect()
}

fn parse_procedure_type(value: &str) -> anyhow::Result<Option<ProcedureType>> {
    use ProcedureType::*;
    Ok(Some(match value.trim() {
        "" => return Ok(None),
        "Ajuste Direto Regime Geral" => DirectAward,
        "Ajuste Direto Regime Geral ao abrigo do artigo 7º da Lei n.º 30/2021, de 21.05" => {
            DirectAwardRuralFires
        }
        "Ajuste direto simplificado" => SimplifiedDirectAward,
        "Ajuste direto simplificado ao abrigo da Lei n.º 30/2021, de 21.05" => {
            SimplifiedDirectAwardSpecialMeasures
        }
        "Consulta Prévia" => PriorConsultation,
        "Consulta prévia ao abrigo do artigo 7º da Lei n.º 30/2021, de 21.05" => {
            PriorConsultationRuralFires
        }
        "Consulta Prévia Simplificada" => SimplifiedPriorConsultation,
        "Concurso público" => PublicTender,
        "Concurso público simplificado" => SimplifiedPublicTender,
        "Concurso limitado por prévia qualificação" => RestrictedTender,
        "Concurso limitado por prévia qualificação simplificado" => SimplifiedRestrictedTender,
        "Procedimento de negociação" => NegotiatedProcedure,
        "Diálogo concorrencial" => CompetitiveDialogue,
        "Parceria para a inovação" => InnovationPartnership,
        "Ao abrigo de acordo-quadro (art.º 258.º)" => FrameworkAgreementArt258,
        "Ao abrigo de acordo-quadro (art.º 259.º)" => FrameworkAgreementArt259,
        "Disponibilização de bens móveis" => GoodsProvision,
        "Serviços sociais e outros serviços específicos" => SocialAndOtherSpecificServices,
        "Concurso de conceção simplificado" => SimplifiedDesignContest,
        "Concurso de ideias simplificado" => SimplifiedIdeasContest,
        "Setores especiais – isenção parte II" => SpecialSectorsExemption,
        "Contratação excluída II" => ExcludedContracting,
        value => anyhow::bail!("Unknown procedure type: {value}"),
    }))
}

fn parse_contract_types(values: Vec<String>) -> anyhow::Result<Vec<ContractType>> {
    values
        .iter()
        .map(|value| parse_contract_type(value))
        .collect()
}

fn parse_contract_type(value: &str) -> anyhow::Result<ContractType> {
    Ok(match value {
        "Aquisição de bens móveis" => ContractType::GoodsAcquisition,
        "Aquisição de serviços" => ContractType::ServicesAcquisition,
        "Concessão de obras públicas" => ContractType::PublicWorksConcession,
        "Concessão de serviços públicos" => ContractType::PublicServicesConcession,
        "Empreitadas de obras públicas" => ContractType::PublicWorks,
        "Locação de bens móveis" => ContractType::GoodsLease,
        "Sociedade" => ContractType::Partnership,
        "Outros" => ContractType::Other,
        // The portal has more details of the type in parentheses. Seen so far:
        // "Outros Tipos (Não Preenchido)"
        // "Outros Tipos (Exploração de atividade comercial)"
        // "Outros Tipos (Concessão de exploração de bens do domínio público)"
        // "Outros Tipos (Prestação do Serviço Universal)"
        // We may want to come back to this and keep the detail so its shown to the user.
        value if value.starts_with("Outros Tipos (") => ContractType::Other,
        _ => anyhow::bail!("Unknown contract type: {value}"),
    })
}

fn api_entities(entities: &Option<Vec<BaseApiEntity>>) -> Vec<Entity> {
    entities
        .iter()
        .flatten()
        .map(|entity| {
            api_entity(
                &entity.nif,
                entity.description.as_deref().unwrap_or_default(),
            )
        })
        .collect()
}

fn api_contestants(contestants: &Option<Vec<Text>>) -> Vec<Entity> {
    contestants
        .iter()
        .flatten()
        .map(|contestant| {
            // contestants come as "NIF-Name", and a hidden NIF is "-", so they come as "--Name"
            // splitting that on the first dash would leave "-Name" as the name, so it's handled first
            let (nif, description) = match contestant.strip_prefix("--") {
                Some(description) => ("", description),
                None => contestant.split_once('-').unwrap_or(("", contestant)),
            };
            api_entity(nif, description)
        })
        .collect()
}

fn api_invitees(invitees: &Option<Vec<BaseApiInvitee>>) -> Vec<Entity> {
    invitees
        .iter()
        .flatten()
        .map(|invitee| {
            api_entity(
                &invitee.nif,
                invitee.description.as_deref().unwrap_or_default(),
            )
        })
        .collect()
}

fn api_entity(nif: &str, description: &str) -> Entity {
    let nif = nif.trim().trim_matches('-').trim().to_string();
    let description = description.trim().to_string();

    let mut hasher = DefaultHasher::new();
    (&nif, &description).hash(&mut hasher);

    Entity {
        // TODO: the API doesn't expose the Portal BASE entity id, so for now the id is a hash of the NIF and name
        id: hasher.finish(),
        nif,
        description,
    }
}

fn portal_entities(entities: &[BasePortalEntity]) -> Vec<Entity> {
    entities
        .iter()
        .map(|entity| Entity {
            id: entity.id,
            description: entity.description.to_string(),
            nif: entity.nif.to_string(),
        })
        .collect()
}

fn portal_documents(documents: &[BasePortalDocument]) -> Vec<Document> {
    documents
        .iter()
        .map(|document| Document {
            id: document.id,
            description: document.description.to_string(),
        })
        .collect()
}

fn parse_portal_date(date: &str) -> anyhow::Result<NaiveDate> {
    NaiveDate::parse_from_str(date, "%d-%m-%Y").with_context(|| format!("Invalid date: {date}"))
}

fn parse_portal_euros(euros: &str) -> anyhow::Result<Currency> {
    // 5.611,10 €
    let cents = euros.trim().trim_end_matches(" €").replace(['.', ','], "");

    let cents = cents
        .parse()
        .with_context(|| format!("Invalid price: {euros}"))?;

    Ok(Currency(cents))
}

fn parse_portal_execution_deadline(deadline: &str) -> anyhow::Result<usize> {
    deadline
        .trim()
        .trim_end_matches(" dias")
        .parse()
        .with_context(|| format!("Invalid execution deadline: {deadline}"))
}

fn parse_portal_list(list: &str, separator: &str) -> Vec<String> {
    list.split(separator)
        .map(str::trim)
        .filter(|s| !s.is_empty())
        .unique()
        .map(str::to_string)
        .collect()
}

fn parse_portal_cpvs(codes: &str) -> anyhow::Result<Vec<Cpv>> {
    parse_portal_list(codes, " | ")
        .iter()
        .map(|code| parse_cpv(code))
        .collect()
}
