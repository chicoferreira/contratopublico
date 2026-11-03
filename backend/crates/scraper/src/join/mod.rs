mod joiner;

use std::time::Duration;

use anyhow::Context;
use common::{db::ContractDatabase, searchdb::SearchDatabase};
use log::{error, info};

use crate::{
    join::joiner::join_contract,
    sources::{
        ContractSource, base_api::BaseApiContract, base_portal::BasePortalContract,
        db::RawContractDatabase,
    },
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
    let mut last_id = 0;
    let mut joined = 0;
    let mut failed = 0;

    loop {
        let queued = raw_contract_db
            .list_queued_after(last_id, BATCH_SIZE)
            .await
            .context("Failed to fetch queued contracts")?;

        let Some(last_entry) = queued.last() else {
            break;
        };
        last_id = last_entry.contract_id;

        let mut contracts = vec![];
        let mut joined_entries = vec![];

        for entry in queued {
            let id = entry.contract_id;

            let api = raw_contract_db
                .get_contract::<BaseApiContract>(ContractSource::BaseApi, id)
                .await
                .with_context(|| format!("Failed to read BASE API contract {id}"))?;

            let portal = raw_contract_db
                .get_contract::<BasePortalContract>(ContractSource::BasePortal, id)
                .await
                .with_context(|| format!("Failed to read Portal BASE contract {id}"))?;

            match join_contract(api, portal) {
                Ok(contract) => {
                    contract_db
                        .insert_contract(&contract)
                        .await
                        .with_context(|| format!("Failed to save contract {id}"))?;

                    contracts.push(contract);
                    joined_entries.push(entry);
                }
                Err(e) => {
                    error!("Failed to join contract {id}:\n{e:?}");
                    failed += 1;
                }
            }
        }

        joined += contracts.len();

        search_db
            .save_contracts(contracts)
            .await
            .context("Failed to save contracts to search")?;

        raw_contract_db
            .dequeue(&joined_entries)
            .await
            .context("Failed to remove joined contracts from the queue")?;
    }

    if joined > 0 || failed > 0 {
        info!("Joined {joined} contracts, {failed} failed");
    }

    Ok(())
}
