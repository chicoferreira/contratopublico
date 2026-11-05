use std::str::FromStr;

use anyhow::Context;
use chrono::{Datelike, Utc};
use cron::Schedule;
use itertools::Itertools;
use log::{error, info};

use crate::sources::{ContractSource, base_api::client::BaseApiClient, db::RawContractDatabase};

// Every hour, on the hour (UTC)
const RECENT_SYNC_SCHEDULE: &str = "0 0 * * * *";
// Every Sunday at 04:00 (UTC)
const FULL_SYNC_SCHEDULE: &str = "0 0 4 * * Sun";
const RECENT_SYNC_LOOKBACK_DAYS: u16 = 2;
const BATCH_SIZE: usize = 1000;
const MAX_FULL_SYNC_DELETIONS: i64 = 10_000;

pub async fn run_sync_task(client: BaseApiClient, raw_contract_database: RawContractDatabase) {
    let recent_schedule = Schedule::from_str(RECENT_SYNC_SCHEDULE).unwrap();
    let full_schedule = Schedule::from_str(FULL_SYNC_SCHEDULE).unwrap();

    loop {
        let next_recent = recent_schedule.upcoming(Utc).next().unwrap();
        let next_full = full_schedule.upcoming(Utc).next().unwrap();
        let next = next_recent.min(next_full);

        info!("Next recent BASE API sync at {next_recent}, next full sync at {next_full}");

        let wait = (next - Utc::now()).to_std().unwrap_or_default();
        tokio::time::sleep(wait).await;

        if next_full <= next_recent {
            if let Err(e) = sync_all(&client, &raw_contract_database).await {
                error!("Failed to download every BASE API contract:\n{e:?}");
            }
        } else if let Err(e) = sync_recent(&client, &raw_contract_database).await {
            error!("Failed to download recent BASE API contracts:\n{e:?}");
        }
    }
}

pub async fn sync_all(
    client: &BaseApiClient,
    raw_contract_database: &RawContractDatabase,
) -> anyhow::Result<()> {
    // The first year that the BASE API has contracts is 2012, so we start from there
    const FIRST_YEAR: u16 = 2012;

    let mut snapshot = raw_contract_database
        .begin_snapshot(ContractSource::BaseApi)
        .await
        .context("Failed to start the BASE API snapshot")?;

    for year in FIRST_YEAR..=Utc::now().year() as u16 {
        let contracts = client
            .fetch_by_year(year)
            .await
            .with_context(|| format!("Failed to fetch contracts of {year}"))?;

        info!("Fetched {} BASE API contracts of {year}", contracts.len());

        // the API repeats a contract once per contracting entity
        // snapshot only keeps one copy of each
        for batch in contracts.chunks(BATCH_SIZE) {
            snapshot
                .add(batch.iter().map(|contract| (contract.id, contract)))
                .await
                .context("Failed to add contracts to the BASE API snapshot")?;
        }
    }

    let deleted = snapshot
        .merge()
        .await
        .context("Failed to merge the BASE API snapshot")?;

    anyhow::ensure!(
        deleted <= MAX_FULL_SYNC_DELETIONS,
        "The BASE API snapshot would delete {deleted} contracts, more than the limit of {MAX_FULL_SYNC_DELETIONS}, so nothing was saved"
    );

    snapshot
        .commit()
        .await
        .context("Failed to save the BASE API snapshot")?;

    info!("Deleted {deleted} contracts that are no longer in the BASE API");

    Ok(())
}

async fn sync_recent(
    client: &BaseApiClient,
    raw_contract_database: &RawContractDatabase,
) -> anyhow::Result<()> {
    let contracts = client
        .fetch_by_previous_days(RECENT_SYNC_LOOKBACK_DAYS)
        .await
        .context("Failed to fetch recent contracts")?;

    info!("Fetched {} recent BASE API contracts", contracts.len());

    // the API repeats a contract once per contracting entity, in identical copies
    let contracts: Vec<_> = contracts
        .into_iter()
        .unique_by(|contract| contract.id)
        .map(|contract| (contract.id, contract))
        .collect();

    for batch in contracts.chunks(BATCH_SIZE) {
        raw_contract_database
            .save_contracts(ContractSource::BaseApi, batch)
            .await
            .context("Failed to save raw contracts in database")?;
    }

    Ok(())
}
