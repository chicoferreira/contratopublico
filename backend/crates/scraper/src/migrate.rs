use std::{
    path::PathBuf,
    sync::Arc,
    time::{Duration, Instant},
};

use common::{Contract, SearchableContract};
use sqlx::PgPool;
use tokio::task::JoinHandle;

pub async fn migrate_contracts_to_postgres(
    contracts_path: PathBuf,
    meili_client: meilisearch_sdk::client::Client,
    pg_pool: PgPool,
) -> anyhow::Result<()> {
    let index = Arc::new(meili_client.index("contracts"));

    let instant = Instant::now();

    let file = std::fs::File::open(&contracts_path)?;
    let reader = std::io::BufReader::new(file);
    let contracts: Arc<Vec<Contract>> = Arc::new(serde_json::from_reader(reader)?);

    log::info!(
        "Loaded {} contracts in {:?}",
        contracts.len(),
        instant.elapsed()
    );

    index
        .delete_all_documents()
        .await?
        .wait_for_completion(&meili_client, None, Some(Duration::from_secs(10000)))
        .await?;

    log::info!("Deleted all documents from meilisearch");

    let contracts_clone = Arc::clone(&contracts);
    let meili_task: JoinHandle<anyhow::Result<()>> = tokio::spawn(async move {
        let chunks = contracts_clone.chunks(1000);
        let size = chunks.len();
        for (i, contracts_chunk) in chunks.enumerate() {
            let instant = Instant::now();
            let contracts_chunk = contracts_chunk
                .into_iter()
                .map(|contract| contract.clone().into())
                .collect::<Vec<SearchableContract>>();

            index.add_documents(&contracts_chunk, Some("id")).await?;

            let duration = instant.elapsed();
            log::info!(
                "Indexed {} contracts in {:?} ({}/{})",
                contracts_chunk.len(),
                duration,
                i + 1,
                size
            );
        }

        Ok(())
    });

    let pg_task: JoinHandle<anyhow::Result<()>> = tokio::spawn(async move {
        for contracts_chunk in contracts.chunks(10) {
            let mut handles: Vec<JoinHandle<anyhow::Result<()>>> =
                Vec::with_capacity(contracts_chunk.len());

            for contract in contracts_chunk {
                let pg_pool = pg_pool.clone();
                let contract = contract.clone();
                let handle = tokio::spawn(async move {
                    let instant = Instant::now();
                    common::db::insert_contract(&contract, &pg_pool).await?;

                    let duration = instant.elapsed();
                    log::info!("Inserted contract {} in {:?}", contract.id, duration);

                    Ok(())
                });

                handles.push(handle);
            }

            for handle in handles {
                let _ = handle.await?;
            }
        }

        Ok(())
    });

    pg_task.await??;
    meili_task.await??;

    Ok(())
}
