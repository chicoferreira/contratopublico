use std::path::PathBuf;

use anyhow::Context;
use common::{Contract, Cpv};
use meilisearch_sdk::documents::DocumentsQuery;

fn migrate_contract_cpv(contract: &serde_json::Value) -> anyhow::Result<Vec<Cpv>> {
    let cpv = contract["cpv"]
        .as_object()
        .context("cpv not present in contract")?;

    let code = cpv["code"]
        .as_str()
        .context("code not present in cpv object")?;

    let designation = cpv["designation"]
        .as_str()
        .context("designation not present in cpv object")?;

    let mut result = vec![];

    let code = code.split(" | ");
    let designation = designation.split(" | ");

    for (code, designation) in code.zip(designation) {
        if code.is_empty() || designation.is_empty() {
            continue;
        }

        result.push(Cpv {
            code: code.to_string(),
            designation: designation.to_string(),
        });
    }

    Ok(result)
}

pub async fn export_old_format_to_json(
    meilisearch_client: meilisearch_sdk::client::Client,
    output_path: PathBuf,
) -> anyhow::Result<()> {
    let mut contracts: Vec<serde_json::Value> = vec![];
    let mut total = 1;
    let mut offset = 0;
    const PER_PAGE: usize = 10000;

    let index = meilisearch_client.index("contracts");
    loop {
        if total <= offset {
            break;
        }

        let mut query = DocumentsQuery::new(&index);
        query.with_limit(PER_PAGE).with_offset(offset);

        let results = index.get_documents_with(&query).await?;

        log::info!(
            "Exporting contracts from {} to {} (total: {})",
            offset,
            offset + results.results.len(),
            total
        );
        total = results.total as usize;
        offset += results.results.len();

        contracts.extend(results.results);
    }

    // migrate CPV separated by '|' to a list of CPV
    let contracts: Vec<Contract> = contracts
        .into_iter()
        .map(|mut contract| {
            let cpvs = migrate_contract_cpv(&contract).unwrap();
            contract["cpvs"] = serde_json::Value::Array(
                cpvs.into_iter()
                    .map(|cpv| serde_json::to_value(cpv).unwrap())
                    .collect(),
            );
            contract
        })
        .map(|contract| serde_json::from_value(contract).unwrap())
        .collect::<Vec<_>>();

    let json = serde_json::to_string_pretty(&contracts)?;
    std::fs::write(&output_path, json)?;

    log::info!(
        "Exported {} contracts to {}",
        contracts.len(),
        output_path.display()
    );

    Ok(())
}
