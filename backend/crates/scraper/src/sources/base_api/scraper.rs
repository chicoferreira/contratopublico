use std::time::Duration;

use anyhow::Context;
use chrono::{Datelike, Utc};
use itertools::Itertools;
use log::{error, info};

use crate::sources::{
    ContractSource,
    base_api::{BaseApiContract, client::BaseApiClient},
    db::RawContractDatabase,
};

const FIRST_YEAR: u16 = 2012;
const RECENT_DAYS: u16 = 2;
const SYNC_INTERVAL: Duration = Duration::from_hours(1);
const BATCH_SIZE: usize = 1000;

pub async fn run_sync_task(client: BaseApiClient, raw_contract_database: RawContractDatabase) {
    // TODO: for now we are downloading every contract every time the service starts, maybe we can do better later
    while let Err(e) = sync_all(&client, &raw_contract_database).await {
        error!("Failed to download every BASE API contract:\n{e:?}");
        tokio::time::sleep(SYNC_INTERVAL).await;
    }

    loop {
        tokio::time::sleep(SYNC_INTERVAL).await;

        if let Err(e) = sync_recent(&client, &raw_contract_database).await {
            error!("Failed to download recent BASE API contracts:\n{e:?}");
        }
    }
}

pub async fn sync_all(
    client: &BaseApiClient,
    raw_contract_database: &RawContractDatabase,
) -> anyhow::Result<()> {
    for year in FIRST_YEAR..=Utc::now().year() as u16 {
        let contracts = client
            .fetch_by_year(year)
            .await
            .with_context(|| format!("Failed to fetch contracts of {year}"))?;

        info!("Fetched {} BASE API contracts of {year}", contracts.len());
        save(raw_contract_database, contracts).await?;
    }

    Ok(())
}

async fn sync_recent(
    client: &BaseApiClient,
    raw_contract_database: &RawContractDatabase,
) -> anyhow::Result<()> {
    let contracts = client
        .fetch_by_previous_days(RECENT_DAYS)
        .await
        .context("Failed to fetch recent contracts")?;

    info!("Fetched {} recent BASE API contracts", contracts.len());
    save(raw_contract_database, contracts).await
}

async fn save(
    raw_contract_database: &RawContractDatabase,
    contracts: Vec<BaseApiContract>,
) -> anyhow::Result<()> {
    // TODO: handle several rows for the same contract id, for now only one of them is kept
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
