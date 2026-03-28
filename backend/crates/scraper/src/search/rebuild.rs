use std::time::Duration;

use anyhow::Context;
use common::{SearchableContract, db::ContractDatabase, searchdb::SearchDatabase};
use futures::{StreamExt, TryStreamExt, stream};
use log::info;
use meilisearch_sdk::client::SwapIndexes;
use tokio::time::Instant;

const FETCH_CONCURRENCY: usize = 10;
const BATCH_SIZE: usize = 5000;

async fn load_contracts_parallel(
    contract_database: &ContractDatabase,
    ids: &[u64],
) -> anyhow::Result<Vec<SearchableContract>> {
    stream::iter(ids.iter().copied().map(|id| {
        let db = contract_database.clone();

        async move {
            let contract = db
                .get_contract(id)
                .await
                .with_context(|| format!("Failed to load contract {id}"))?
                .with_context(|| format!("Contract {id} disappeared during search rebuild"))?;

            anyhow::Ok(SearchableContract::from(contract))
        }
    }))
    .buffer_unordered(FETCH_CONCURRENCY)
    .try_collect()
    .await
}

pub async fn rebuild_search_index(
    contract_database: &ContractDatabase,
    search_database: &SearchDatabase,
) -> anyhow::Result<()> {
    let client = search_database.client();

    let indexes = client
        .get_indexes()
        .await
        .context("Failed to get indexes")?;

    for index in indexes.results {
        let index_name = index.uid;
        if index_name.starts_with("migration_") {
            info!("Found unremoved migration index '{index_name}'. Deleting...");
            client
                .delete_index(&index_name)
                .await
                .context("Failed to delete migration index")?
                .wait_for_completion(client, None, Some(Duration::from_mins(30)))
                .await?;
        }
    }

    let current_time_str = chrono::Utc::now().format("%Y%m%d%H%M%S").to_string();
    let index_name = format!("migration_{current_time_str}");
    let index = client.index(&index_name);

    info!("Rebuilding search index into {index_name}");

    let mut last_id = 0_u64;
    let mut total_indexed = 0_usize;

    let mut add_documents_tasks = Vec::new();

    loop {
        let instant = Instant::now();
        let ids = contract_database
            .list_contract_ids_after(last_id, BATCH_SIZE)
            .await
            .context("Failed to list contract ids for search rebuild")?;

        if ids.is_empty() {
            break;
        }

        let contracts = load_contracts_parallel(contract_database, &ids).await?;

        let task = index
            .add_documents(&contracts, Some("id"))
            .await
            .with_context(|| {
                format!("Failed to save batch ending at id {}", ids.last().unwrap())
            })?;

        add_documents_tasks.push(task);

        last_id = ids[ids.len() - 1];
        total_indexed += contracts.len();

        info!(
            "Indexed {total_indexed} contracts into {index_name} in {:?}",
            instant.elapsed()
        );
    }

    info!("Waiting for {total_indexed} add_documents tasks to complete...");
    for task in add_documents_tasks {
        task.wait_for_completion(client, None, Some(Duration::from_hours(1)))
            .await
            .with_context(|| format!("Failed to complete add_documents task"))?;
    }

    let swap_indexes = SwapIndexes {
        indexes: (index_name.clone(), "contracts".to_string()),
    };

    client
        .swap_indexes([&swap_indexes])
        .await
        .with_context(|| format!("Failed to swap rebuilt index {index_name} into contracts"))?
        .wait_for_completion(client, None, Some(Duration::from_mins(30)))
        .await?;

    info!("Swapped rebuilt index {index_name} into contracts");

    client
        .delete_index(&index_name)
        .await
        .with_context(|| format!("Failed to delete old contracts index at {index_name}"))?
        .wait_for_completion(client, None, Some(Duration::from_mins(30)))
        .await?;

    info!("Deleted old contracts index now stored at {index_name}");
    info!("Finished rebuilding search index {index_name} with {total_indexed} contracts");

    Ok(())
}
