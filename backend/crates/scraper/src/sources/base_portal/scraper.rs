use crate::{
    metrics::{self, ContractFailure},
    sources::{
        ContractSource,
        base_portal::{
            self, BasePortalContract,
            client::{BasePortalClient, ContractSort},
            store::Store,
            throttled_client::ThrottledClient,
        },
        db::RawContractDatabase,
    },
};
use anyhow::Context;
use governor::Quota;
use log::{error, info};
use std::{collections::VecDeque, time::Duration};
use tokio::time::Instant;

pub const MAX_PAGE_SIZE: usize = 50;
const CONTRACT_SORT_ORDER: ContractSort = base_portal::client::ContractSort {
    method: base_portal::client::ContractSortMethod::Id,
    order: base_portal::client::SortOrder::Ascending,
};

const MAX_CONTRACT_ATTEMPTS: usize = 3;

fn max_request_quota() -> Quota {
    Quota::with_period(Duration::from_secs(2)).unwrap()
}

pub async fn run_scrape_task(
    client: BasePortalClient,
    store: Store,
    raw_contract_db: RawContractDatabase,
    interval: Duration,
) {
    let client = ThrottledClient::new(client, max_request_quota());

    let mut listener = match raw_contract_db.listen_portal_refetches().await {
        Ok(listener) => listener,
        Err(e) => {
            error!("Failed to listen to the portal refetch queue:\n{e:?}");
            return;
        }
    };

    let mut failed_refetches = Vec::new();
    let mut scan: Option<PageScan> = None;
    let mut scan_start = Instant::now();
    let mut next_scan = Instant::now();

    loop {
        refetch_queued_contracts(&client, &raw_contract_db, &mut failed_refetches).await;

        if let Some(current_scan) = &mut scan {
            match current_scan.step(&client, &store, &raw_contract_db).await {
                Ok(true) => {}
                Ok(false) => {
                    metrics::run_finished(true, scan_start.elapsed());
                    metrics::next_run_in(interval);
                    next_scan = Instant::now() + interval;
                    scan = None;
                }
                Err(e) => error!("{e:?}"),
            }
            continue;
        }

        tokio::select! {
            result = listener.recv() => {
                // A notification was received, so jump to the start of the loop to refetch the queued contracts again.
                if let Err(e) = result {
                    error!("Failed to wait for the portal refetch queue:\n{e:?}");
                    tokio::time::sleep_until(next_scan).await;
                }
            }
            _ = tokio::time::sleep_until(next_scan) => {
                metrics::run_started();
                scan_start = Instant::now();
                failed_refetches.clear();
                scan = Some(PageScan::default());
            }
        }
    }
}

/// Fetches again the contracts that changed in the BASE API, until the queue is empty.
///
/// A contract that fails is kept in the queue and skipped until the next scan.
async fn refetch_queued_contracts(
    client: &ThrottledClient,
    raw_contract_db: &RawContractDatabase,
    failed_refetches: &mut Vec<i64>,
) {
    loop {
        let entry = match raw_contract_db.next_portal_refetch(failed_refetches).await {
            Ok(Some(entry)) => entry,
            Ok(None) => return,
            Err(e) => {
                error!("Failed to read the portal refetch queue:\n{e:?}");
                return;
            }
        };

        if let Err(e) = fetch_and_save(client, raw_contract_db, entry.contract_id as u64).await {
            error!("{e:?}");
            failed_refetches.push(entry.contract_id);
            continue;
        }

        if let Err(e) = raw_contract_db.dequeue_portal_refetch(entry).await {
            error!("Failed to remove contract from the portal refetch queue:\n{e:?}");
            failed_refetches.push(entry.contract_id);
        }
    }
}

#[derive(Default)]
struct PageScan {
    total_pages: Option<usize>,
    next_page: usize,
    page: usize,
    ids: VecDeque<u64>,
}

impl PageScan {
    /// Fetches the next contract of the current page, or the next page.
    ///
    /// Returns `true` if there is more to scan.
    async fn step(
        &mut self,
        client: &ThrottledClient,
        store: &Store,
        raw_contract_db: &RawContractDatabase,
    ) -> anyhow::Result<bool> {
        while let Some(id) = self.ids.pop_front() {
            if store.already_exists(id, self.page).await {
                info!("Contract {id} already exists, skipping...");
                continue;
            }

            fetch_and_save(client, raw_contract_db, id).await?;
            store.mark_saved(id, self.page, MAX_PAGE_SIZE)?;
            return Ok(true);
        }

        if self
            .total_pages
            .is_some_and(|total_pages| self.next_page >= total_pages)
        {
            return Ok(false);
        }

        self.page = store.get_next_page_to_query(self.next_page);
        metrics::page_progress(self.page, self.total_pages);

        let total_pages_str = self
            .total_pages
            .map(|s| s.to_string())
            .unwrap_or("?".to_string());

        info!("Fetching page {}/{total_pages_str}...", self.page);

        let response = client
            .fetch_page(CONTRACT_SORT_ORDER, self.page, MAX_PAGE_SIZE)
            .await
            .with_context(|| format!("Failed to fetch IDs page {}", self.page))?;

        let len = response.items.len();
        info!("Fetched page {} with {len} contracts", self.page);

        self.ids
            .extend(response.items.iter().map(|contract| contract.id));

        let new_total_pages = response.total / MAX_PAGE_SIZE;
        if self
            .total_pages
            .is_none_or(|total_pages| total_pages < new_total_pages)
        {
            self.total_pages = Some(new_total_pages);
        }

        self.next_page = self.page + 1;
        Ok(true)
    }
}

async fn fetch_and_save(
    client: &ThrottledClient,
    raw_contract_db: &RawContractDatabase,
    id: u64,
) -> anyhow::Result<()> {
    let Some(contract) = fetch_contract(client, id).await? else {
        info!("Contract {id} no longer exists in the portal, deleting it...");
        return raw_contract_db
            .delete_contract(ContractSource::BasePortal, id)
            .await
            .with_context(|| format!("Failed to delete contract {id}"));
    };

    raw_contract_db
        .save_contracts(ContractSource::BasePortal, &[(contract.id, contract)])
        .await
        .inspect_err(|_| metrics::contract_failed(ContractFailure::Save))
        .with_context(|| format!("Failed to save details for ID {id}"))?;

    metrics::contract_saved();
    Ok(())
}

/// Returns `None` if the portal doesn't have the contract.
async fn fetch_contract(
    client: &ThrottledClient,
    id: u64,
) -> anyhow::Result<Option<BasePortalContract>> {
    for attempt in 1..=MAX_CONTRACT_ATTEMPTS {
        info!("Fetching details for contract {id}...");
        match client.get_contract_details(id).await {
            Ok(contract) => {
                info!("Fetched details for contract {id}");
                return Ok(contract);
            }
            Err(e) => error!(
                "Failed to fetch details for ID {id} (attempt {attempt}/{MAX_CONTRACT_ATTEMPTS}):\n{e:?}"
            ),
        }
    }

    metrics::contract_failed(ContractFailure::Fetch);
    anyhow::bail!("Giving up on contract {id} after {MAX_CONTRACT_ATTEMPTS} failed attempts")
}
