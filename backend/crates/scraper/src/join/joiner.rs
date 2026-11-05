use std::hash::{DefaultHasher, Hash, Hasher};

use anyhow::Context;
use chrono::NaiveDate;
use common::{Contract, Cpv, Currency, Document, Entity};
use itertools::Itertools;

use crate::sources::{
    base_api::{BaseApiContract, BaseApiCpv, BaseApiEntity},
    base_portal::{BasePortalContract, BasePortalDocument, BasePortalEntity},
};

const DIRECT_AWARD_NOT_APPLICABLE: &str = "Não aplicável";

pub fn join_contract(sources: Sources) -> anyhow::Result<Contract> {
    Ok(Contract {
        id: sources.api_first(|c| c.id, |c| c.id),
        contracting_procedure_type: sources.api_first(
            |c| c.procedure_type.clone(),
            |c| c.contracting_procedure_type.clone(),
        ),
        publication_date: sources.try_api_first(
            |c| parse_api_date(&c.publication_date),
            |c| parse_portal_date(&c.publication_date),
        )?,
        signing_date: sources.try_api_first_option(
            |c| parse_optional_api_date(&c.signing_date),
            |c| c.signing_date.as_deref().map(parse_portal_date).transpose(),
        )?,
        ccp: sources.portal_or_default(|c| c.ccp),
        object_brief_description: sources.api_first(
            |c| c.object_brief_description.clone(),
            |c| c.object_brief_description.clone(),
        ),
        initial_contractual_price: sources.try_api_first(
            |c| Ok(euros_to_currency(c.initial_contractual_price)),
            |c| parse_portal_euros(&c.initial_contractual_price),
        )?,
        description: sources
            .api_first_option(|c| non_empty(&c.description), |c| c.description.clone()),
        contracting: sources.portal_first(
            |c| portal_entities(c.contracting.as_deref().unwrap_or_default()),
            |c| api_entities(&c.contracting),
        ),
        contracted: sources.portal_first(
            |c| portal_entities(c.contracted.as_deref().unwrap_or_default()),
            |c| api_entities(&c.contracted),
        ),
        cpvs: sources.try_api_first(
            |c| Ok(api_cpvs(&c.cpvs)),
            |c| parse_portal_cpvs(&c.cpvs, &c.cpvs_designation),
        )?,
        regime: sources.api_first_option(|c| non_empty(&c.regime), |c| c.regime.clone()),
        contract_status: sources.portal_or_default(|c| c.contract_status.clone()),
        non_written_contract_justification_types: sources.portal_first(
            |c| parse_portal_list(&c.non_written_contract_justification_types, "<br/>"),
            |c| api_list(&c.non_written_contract_justification_types),
        ),
        contract_types: sources.api_first(
            |c| api_list(&c.contract_types),
            |c| parse_portal_list(&c.contract_types, "<br/>"),
        ),
        execution_deadline_days: sources.try_api_first(
            |c| parse_api_execution_deadline(c.execution_deadline_days),
            |c| parse_portal_execution_deadline(&c.execution_deadline),
        )?,
        execution_places: sources.api_first(
            |c| api_list(&c.execution_places),
            |c| parse_portal_list(&c.execution_place, "<BR/>"),
        ),
        contract_fundamentation_type: sources.api_first(
            |c| c.contract_fundamentation_type.clone(),
            |c| c.contract_fundamentation_type.clone(),
        ),
        contestants: sources.portal_first(
            |c| portal_entities(&c.contestants),
            |c| api_contestants(&c.contestants),
        ),
        invitees: sources.portal_or_default(|c| portal_entities(&c.invitees)),
        documents: sources.portal_or_default(|c| portal_documents(&c.documents)),
        contracting_procedure_url: sources.api_first_option(
            |c| non_empty(&c.contracting_procedure_url),
            |c| c.contracting_procedure_url.clone(),
        ),
        announcement_id: sources.portal_or_default(|c| usize::try_from(c.announcement_id).ok()),
        direct_award_fundamentation_type: sources.portal_first_option(
            |c| {
                Some(c.direct_award_fundamentation_type.clone())
                    .filter(|value| value != DIRECT_AWARD_NOT_APPLICABLE)
            },
            |c| non_empty(&c.direct_award_fundamentation_type),
        ),
        observations: sources
            .api_first_option(|c| non_empty(&c.observations), |c| c.observations.clone()),
        end_of_contract_type: sources.portal_first_option(
            |c| c.end_of_contract_type.clone(),
            |c| non_empty(&c.end_of_contract_type),
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
        causes_deadline_change: sources.portal_or_default(|c| c.causes_deadline_change.clone()),
        causes_price_change: sources.portal_or_default(|c| c.causes_price_change.clone()),
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

fn non_empty(value: &str) -> Option<String> {
    (!value.trim().is_empty()).then(|| value.to_string())
}

fn parse_api_date(date: &str) -> anyhow::Result<NaiveDate> {
    NaiveDate::parse_from_str(date, "%Y-%m-%d").with_context(|| format!("Invalid date: {date}"))
}

fn parse_optional_api_date(date: &str) -> anyhow::Result<Option<NaiveDate>> {
    (!date.is_empty()).then(|| parse_api_date(date)).transpose()
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

fn api_cpvs(cpvs: &Option<Vec<BaseApiCpv>>) -> Vec<Cpv> {
    cpvs.iter()
        .flatten()
        .unique_by(|cpv| &cpv.code)
        .map(|cpv| Cpv {
            code: cpv.code.trim().to_string(),
            designation: cpv.designation.trim().to_string(),
        })
        .collect()
}

fn api_list(list: &Option<Vec<String>>) -> Vec<String> {
    list.iter()
        .flatten()
        .map(|item| item.trim().to_string())
        .filter(|item| !item.is_empty())
        .unique()
        .collect()
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

fn api_contestants(contestants: &Option<Vec<String>>) -> Vec<Entity> {
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
            description: entity.description.clone(),
            nif: entity.nif.clone(),
        })
        .collect()
}

fn portal_documents(documents: &[BasePortalDocument]) -> Vec<Document> {
    documents
        .iter()
        .map(|document| Document {
            id: document.id,
            description: document.description.clone(),
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

fn parse_portal_cpvs(codes: &str, designations: &str) -> anyhow::Result<Vec<Cpv>> {
    if codes.is_empty() && designations.is_empty() {
        return Ok(vec![]);
    }

    let code_split: Vec<_> = codes.split(" | ").collect();
    let designation_split: Vec<_> = designations.split(" | ").collect();

    anyhow::ensure!(
        code_split.len() == designation_split.len(),
        "Mismatched number of codes and designations"
    );

    Ok(code_split
        .into_iter()
        .zip(designation_split)
        .map(|(code, designation)| Cpv {
            code: code.trim().to_string(),
            designation: designation.trim().to_string(),
        })
        .collect())
}
