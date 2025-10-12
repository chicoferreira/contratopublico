use std::path::PathBuf;

use common::Contract;
use meilisearch_sdk::documents::DocumentsQuery;

pub async fn export_old_format_to_json(
    meilisearch_client: meilisearch_sdk::client::Client,
    output_path: PathBuf,
) -> anyhow::Result<()> {
    let mut contracts: Vec<Contract> = vec![];
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

    // filter empty cpvs (migration)
    for contract in &mut contracts {
        contract.cpv = contract
            .cpv
            .clone()
            .filter(|cpv| !cpv.code.is_empty() && !cpv.designation.is_empty());
    }

    let json = serde_json::to_string_pretty(&contracts)?;
    std::fs::write(&output_path, json)?;

    log::info!(
        "Exported {} contracts to {}",
        contracts.len(),
        output_path.display()
    );

    Ok(())
}
