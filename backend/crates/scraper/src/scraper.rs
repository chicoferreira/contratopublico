use crate::{
    base_gov::{
        self,
        client::{BaseGovClient, ContractSort},
        throttled_client::{MAX_CONSECUTIVE_FAILURES, ThrottledClient},
    },
    metrics::{self, ContractFailure},
    store::Store,
};
use governor::Quota;
use log::{error, info};
use std::{collections::VecDeque, sync::Arc, time::Duration};
use tokio::time::Instant;

pub const MAX_PAGE_SIZE: usize = 50;
const CONTRACT_SORT_ORDER: ContractSort = base_gov::client::ContractSort {
    method: base_gov::client::ContractSortMethod::Id,
    order: base_gov::client::SortOrder::Ascending,
};

const MAX_CONCURRENT_REQUESTS: usize = 1;
const MAX_CONTRACT_ATTEMPTS: usize = 3;

fn max_request_quota() -> Quota {
    Quota::with_period(Duration::from_secs(2)).unwrap()
}

pub async fn scrape(store: Arc<Store>, base_gov_client: BaseGovClient) {
    let start = Instant::now();
    metrics::run_started();

    let client = ThrottledClient::new(
        base_gov_client,
        MAX_CONCURRENT_REQUESTS,
        max_request_quota(),
    );

    let (id_tx, id_rx) = tokio::sync::mpsc::channel(MAX_CONCURRENT_REQUESTS);

    let fetch_task = run_fetch_ids_task(&client, store.clone(), id_tx);
    let details_task = run_fetch_details_task(&client, store, id_rx);

    let (completed, ()) = tokio::join!(fetch_task, details_task);

    metrics::run_finished(completed, start.elapsed());
}

struct ContractLocation {
    id: u64,
    page: usize,
    retries: usize,
}

async fn run_fetch_ids_task(
    client: &ThrottledClient,
    store: Arc<Store>,
    id_tx: tokio::sync::mpsc::Sender<ContractLocation>,
) -> bool {
    let mut completed = false;
    let mut total_pages = None;
    let mut current_page = 0_usize;

    loop {
        if total_pages.is_some_and(|total_pages| current_page >= total_pages) {
            completed = true;
            break;
        }

        current_page = store.get_next_page_to_query(current_page);
        metrics::page_progress(current_page, total_pages);

        let total_pages_str = total_pages
            .map(|s| s.to_string())
            .unwrap_or("?".to_string());

        info!("Fetching page {current_page}/{total_pages_str}...");

        let Some(response) = client
            .fetch_page(CONTRACT_SORT_ORDER, current_page, MAX_PAGE_SIZE)
            .await
        else {
            error!("Requests failed {MAX_CONSECUTIVE_FAILURES} consecutive times, stopping");
            break;
        };

        let response = match response {
            Ok(response) => response,
            Err(e) => {
                error!("Failed to fetch IDs page {current_page}:\n{e:?}");
                continue;
            }
        };

        info!(
            "Fetched page {current_page} with {} contracts",
            response.items.len()
        );

        let minimal_contracts = response.items;

        for minimal_contract in minimal_contracts {
            // this will block this task until the receiver needs more ids to fetch because
            // the tokio::sync::mpsc::Sender has a buffer which will create backpressure when full
            let _ = id_tx
                .send(ContractLocation {
                    id: minimal_contract.id,
                    page: current_page,
                    retries: 0,
                })
                .await;
        }

        let new_total_pages = response.total / MAX_PAGE_SIZE;
        if total_pages.is_none_or(|total_pages| total_pages < new_total_pages) {
            total_pages = Some(new_total_pages);
        }

        current_page += 1;
    }

    completed
}

async fn run_fetch_details_task(
    client: &ThrottledClient,
    store: Arc<Store>,
    mut id_rx: tokio::sync::mpsc::Receiver<ContractLocation>,
) {
    let mut retry_queue = VecDeque::new();

    loop {
        let next = match id_rx.try_recv().ok().or_else(|| retry_queue.pop_front()) {
            Some(contract_location) => Some(contract_location),
            None => id_rx.recv().await,
        };
        let Some(ContractLocation { id, page, retries }) = next else {
            // The channel closed and there are no more retries, so we can exit the loop
            break;
        };

        if retries >= MAX_CONTRACT_ATTEMPTS {
            error!("Giving up on contract {id} after {retries} failed attempts");
            metrics::contract_failed(ContractFailure::Fetch);
            continue;
        }

        if store.already_exists(id, page).await {
            info!("Contract {id} already exists, skipping...");
            continue;
        }

        info!("Fetching details for contract {id}...");
        let Some(response) = client.get_contract_details(id).await else {
            break;
        };

        let contract = match response {
            Ok(response) => response,
            Err(e) => {
                error!("Failed to fetch details for ID {id}:\n{e:?}");
                retry_queue.push_back(ContractLocation {
                    id,
                    page,
                    retries: retries + 1,
                });
                continue;
            }
        };

        let contract = contract.into();
        info!("Fetched details for contract {id}");

        match store
            .save_scraped_contract(contract, page, MAX_PAGE_SIZE)
            .await
        {
            Ok(()) => metrics::contract_saved(),
            Err(e) => {
                error!("Failed to save details for ID {id}:\n{:?}", e);
                metrics::contract_failed(ContractFailure::Save);
            }
        }
    }
}
