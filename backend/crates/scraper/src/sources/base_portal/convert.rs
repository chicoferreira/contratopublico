use anyhow::Context;
use chrono::NaiveDate;
use common::{Contract, Cpv, Currency, Document, Entity};
use itertools::Itertools;

use crate::sources::base_portal::{BasePortalContract, BasePortalDocument, BasePortalEntity};

const DIRECT_AWARD_NOT_APPLICABLE: &str = "Não aplicável";

impl TryFrom<BasePortalContract> for Contract {
    type Error = anyhow::Error;

    fn try_from(contract: BasePortalContract) -> anyhow::Result<Contract> {
        Ok(Contract {
            id: contract.id,
            contracting_procedure_type: contract.contracting_procedure_type,
            publication_date: parse_date(&contract.publication_date)?,
            signing_date: contract
                .signing_date
                .as_deref()
                .map(parse_date)
                .transpose()?,
            ccp: contract.ccp,
            contracted: entities(contract.contracted.unwrap_or_default()),
            contracting: entities(contract.contracting.unwrap_or_default()),
            object_brief_description: contract.object_brief_description,
            initial_contractual_price: parse_euros(&contract.initial_contractual_price)?,
            description: contract.description,
            cpvs: parse_cpvs(&contract.cpvs, &contract.cpvs_designation)?,
            regime: contract.regime,
            contract_status: contract.contract_status,
            non_written_contract_justification_types: parse_list(
                &contract.non_written_contract_justification_types,
                "<br/>",
            ),
            contract_types: parse_list(&contract.contract_types, "<br/>"),
            execution_deadline_days: parse_execution_deadline(&contract.execution_deadline)?,
            execution_places: parse_execution_places(&contract.execution_place),
            contract_fundamentation_type: contract.contract_fundamentation_type,
            contestants: entities(contract.contestants),
            invitees: entities(contract.invitees),
            documents: contract.documents.into_iter().map(Into::into).collect(),
            contracting_procedure_url: contract.contracting_procedure_url,
            announcement_id: usize::try_from(contract.announcement_id).ok(),
            direct_award_fundamentation_type: Some(contract.direct_award_fundamentation_type)
                .filter(|value| value != DIRECT_AWARD_NOT_APPLICABLE),
            observations: contract.observations,
            end_of_contract_type: contract.end_of_contract_type,
            close_date: contract.close_date.as_deref().map(parse_date).transpose()?,
            total_effective_price: contract
                .total_effective_price
                .as_deref()
                .map(parse_euros)
                .transpose()?,
            causes_deadline_change: contract.causes_deadline_change,
            causes_price_change: contract.causes_price_change,
        })
    }
}

impl From<BasePortalEntity> for Entity {
    fn from(entity: BasePortalEntity) -> Entity {
        Entity {
            id: entity.id,
            description: entity.description,
            nif: entity.nif,
        }
    }
}

impl From<BasePortalDocument> for Document {
    fn from(document: BasePortalDocument) -> Document {
        Document {
            id: document.id,
            description: document.description,
        }
    }
}

fn entities(entities: Vec<BasePortalEntity>) -> Vec<Entity> {
    entities.into_iter().map(Into::into).collect()
}

fn parse_date(date: &str) -> anyhow::Result<NaiveDate> {
    NaiveDate::parse_from_str(date, "%d-%m-%Y").with_context(|| format!("Invalid date: {date}"))
}

fn parse_euros(euros: &str) -> anyhow::Result<Currency> {
    // 5.611,10 €
    let cents = euros
        .trim()
        .trim_end_matches(" €")
        .replace(".", "")
        .replace(",", "");

    let cents = cents
        .parse()
        .with_context(|| format!("Invalid price: {euros}"))?;

    Ok(Currency(cents))
}

fn parse_execution_deadline(deadline: &str) -> anyhow::Result<usize> {
    deadline
        .trim()
        .trim_end_matches(" dias")
        .parse()
        .with_context(|| format!("Invalid execution deadline: {deadline}"))
}

fn parse_execution_places(places: &str) -> Vec<String> {
    places
        .split("<BR/>")
        .filter(|s| !s.is_empty())
        .map(str::trim)
        .unique()
        .map(str::to_string)
        .collect_vec()
}

fn parse_list(list: &str, separator: &str) -> Vec<String> {
    list.split(separator)
        .map(str::trim)
        .filter(|s| !s.is_empty())
        .unique()
        .map(str::to_string)
        .collect()
}

fn parse_cpvs(codes: &str, designations: &str) -> anyhow::Result<Vec<Cpv>> {
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
