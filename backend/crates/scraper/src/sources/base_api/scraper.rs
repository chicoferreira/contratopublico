use std::str::FromStr;

use anyhow::Context;
use chrono::{Datelike, Utc};
use cron::Schedule;
use itertools::Itertools;
use log::{error, info};

use crate::sources::{
    ContractSource,
    base_api::{BaseApiContract, client::BaseApiClient},
    db::RawContractDatabase,
};

// Every hour, on the hour (UTC)
const RECENT_SYNC_SCHEDULE: &str = "0 0 * * * *";
// Every Sunday at 04:00 (UTC)
const FULL_SYNC_SCHEDULE: &str = "0 0 4 * * Sun";
const RECENT_SYNC_LOOKBACK_DAYS: u16 = 2;
const BATCH_SIZE: usize = 1000;

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
        let contracts = dedup_contracts(contracts);

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

    let contracts: Vec<_> = dedup_contracts(contracts)
        .into_iter()
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

/// The API returns several entries for the same contract id, so this keeps only the one with the
/// highest values of the fields that differ between them, in alphabetical order, which makes the
/// pick stable between fetches.
///
/// This is not the most elegant solution, but it works for now.
/// See https://github.com/chicoferreira/contratopublico/issues/72 for more details.
fn dedup_contracts(contracts: Vec<BaseApiContract>) -> Vec<BaseApiContract> {
    contracts
        .into_iter()
        .into_grouping_map_by(|contract| contract.id)
        .max_by(|_, a, b| {
            (&a.ambient_criteria, &a.award_criterion)
                .cmp(&(&b.ambient_criteria, &b.award_criterion))
        })
        .into_values()
        .collect()
}
