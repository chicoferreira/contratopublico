mod joiner;

use std::time::Duration;

use anyhow::Context;
use common::{db::ContractDatabase, searchdb::SearchDatabase};
use log::{error, info};

use crate::{
    join::joiner::{Sources, join_contract},
    sources::db::{QueuedContract, RawContractDatabase},
};

const RETRY_INTERVAL: Duration = Duration::from_mins(1);
const BATCH_SIZE: i64 = 1000;

pub async fn run_join_task(
    raw_contract_db: RawContractDatabase,
    contract_db: ContractDatabase,
    search_db: SearchDatabase,
) {
    let mut listener = match raw_contract_db.listen_join_queue().await {
        Ok(listener) => listener,
        Err(e) => {
            error!("Failed to listen to the join queue:\n{e:?}");
            return;
        }
    };

    loop {
        if let Err(e) = join_pending(&raw_contract_db, &contract_db, &search_db).await {
            error!("Failed to join contracts:\n{e:?}");
            tokio::time::sleep(RETRY_INTERVAL).await;
            continue;
        }

        if let Err(e) = listener.try_recv().await {
            error!("Failed to wait for the join queue:\n{e:?}");
            tokio::time::sleep(RETRY_INTERVAL).await;
        }
    }
}

async fn join_pending(
    raw_contract_db: &RawContractDatabase,
    contract_db: &ContractDatabase,
    search_db: &SearchDatabase,
) -> anyhow::Result<()> {
    let mut last_id = i64::MAX;
    let mut joined = 0;
    let mut deleted = 0;
    let mut failed = 0;

    loop {
        // Newest first, so recent contracts don't wait behind a large backlog
        let queued = raw_contract_db
            .list_queued_before(last_id, BATCH_SIZE)
            .await
            .context("Failed to fetch queued contracts")?;

        let Some(last) = queued.last() else {
            break;
        };
        last_id = last.entry.contract_id;

        let mut contracts = vec![];
        let mut deleted_ids = vec![];
        let mut joined_entries = vec![];

        for QueuedContract { entry, api, portal } in queued {
            let id = entry.contract_id;

            match Sources::new(&api, &portal) {
                None => deleted_ids.push(id),
                Some(sources) => match join_contract(sources) {
                    Ok(contract) => contracts.push(contract),
                    Err(e) => {
                        error!("Failed to join contract {id}:\n{e:?}");
                        failed += 1;
                        continue;
                    }
                },
            }

            joined_entries.push(entry);
        }

        contract_db
            .insert_contracts(&contracts)
            .await
            .context("Failed to save contracts")?;

        joined += contracts.len();
        deleted += deleted_ids.len();

        search_db
            .save_contracts(contracts)
            .await
            .context("Failed to save contracts to search")?;

        if !deleted_ids.is_empty() {
            contract_db
                .delete_contracts(&deleted_ids)
                .await
                .context("Failed to delete contracts")?;

            search_db
                .delete_contracts(&deleted_ids)
                .await
                .context("Failed to delete contracts from search")?;
        }

        raw_contract_db
            .dequeue(&joined_entries)
            .await
            .context("Failed to remove joined contracts from the queue")?;
    }

    if joined > 0 || deleted > 0 || failed > 0 {
        info!("Joined {joined} contracts, deleted {deleted}, {failed} failed");
    }

    Ok(())
}
