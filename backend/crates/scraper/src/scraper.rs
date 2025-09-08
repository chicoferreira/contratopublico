use crate::{
    base_gov::{
        self,
        client::{BaseGovClient, ContractSort},
    },
    scraper::throttle::Throttler,
    store::Store,
};
use governor::Quota;
use log::{error, info, warn};
use std::{num::NonZeroU32, sync::Arc};
use tokio::task::JoinHandle;

const MAX_PAGE_SIZE: usize = 50;
const CONTRACT_SORT_ORDER: ContractSort = base_gov::client::ContractSort {
    method: base_gov::client::ContractSortMethod::Id,
    order: base_gov::client::SortOrder::Ascending,
};

// Max consecutive failures when the API keeps failing since the start making it so we don't know the number of pages to scrape
const MAX_CONSECUTIVE_FAILURES: usize = 10;

const MAX_CONCURRENT_REQUESTS: usize = 5;
const MAX_REQUEST_QUOTA: Quota = Quota::per_second(NonZeroU32::new(5).unwrap());

pub async fn scrape(store: Arc<Store>) {
    let client = Arc::new(BaseGovClient::new());
    let throttler = Arc::new(Throttler::new(MAX_CONCURRENT_REQUESTS, MAX_REQUEST_QUOTA));

    let (id_tx, id_rx) = tokio::sync::mpsc::channel(MAX_CONCURRENT_REQUESTS);

    let fetch_task = run_fetch_ids_task(
        client.clone(),
        store.clone(),
        throttler.clone(),
        id_tx.clone(),
    );
    let details_task = run_fetch_details_task(client, store, throttler, id_tx, id_rx);

    tokio::join!(fetch_task, details_task);
}

struct ContractLocation {
    id: usize,
    page: usize,
}

async fn run_fetch_ids_task(
    client: Arc<BaseGovClient>,
    store: Arc<Store>,
    throttler: Arc<Throttler>,
    id_sender: tokio::sync::mpsc::Sender<ContractLocation>,
) {
    let mut total_pages = None;
    let mut consecutive_failures = 0_usize;
    let mut current_page = 0_usize;

    loop {
        let should_continue = match total_pages {
            Some(total_pages) => current_page < total_pages,
            None => consecutive_failures < MAX_CONSECUTIVE_FAILURES,
        };

        if !should_continue {
            break;
        }

        current_page = store.get_next_page_to_query(current_page);

        let total_pages_str = total_pages
            .map(|s| s.to_string())
            .unwrap_or("?".to_string());

        info!("Fetching page {current_page}/{total_pages_str}...");

        let _ = throttler.throttle();
        let response = client
            .fetch_page(CONTRACT_SORT_ORDER, current_page, MAX_PAGE_SIZE)
            .await;

        let response = match response {
            Ok(response) => response,
            Err(e) => {
                error!("Failed to fetch IDs page {current_page}:\n{e:?}");
                consecutive_failures += 1;
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
            let _ = id_sender
                .send(ContractLocation {
                    id: minimal_contract.id,
                    page: current_page,
                })
                .await;
        }

        let new_total_pages = response.total / MAX_PAGE_SIZE;
        if total_pages.is_none_or(|total_pages| total_pages < new_total_pages) {
            total_pages = Some(new_total_pages);
        }

        consecutive_failures = 0;
        current_page += 1;
    }

    // when this finishes id_sender is dropped, and so run_fetch_details_task()
    // will return None at the next recv()
}

async fn run_fetch_details_task(
    client: Arc<BaseGovClient>,
    store: Arc<Store>,
    throttler: Arc<Throttler>,
    id_sender: tokio::sync::mpsc::Sender<ContractLocation>,
    mut id_receiver: tokio::sync::mpsc::Receiver<ContractLocation>,
) {
    let mut handles: Vec<JoinHandle<()>> = Vec::new();

    while let Some(ContractLocation { id, page }) = id_receiver.recv().await {
        if store.already_exists(id, page).await {
            warn!("Contract {id} already exists, skipping...");
            continue;
        }

        handles.retain(|task| !task.is_finished());

        let permit = throttler.throttle().await;
        let client = Arc::clone(&client);
        let store = Arc::clone(&store);
        let id_sender = id_sender.clone();

        let handle = tokio::spawn(async move {
            let _permit = permit; // hold permit until task ends

            info!("Fetching details for contract {id}...");
            let response = client.get_contract_details(id).await;

            let contract = match response {
                Ok(response) => response,
                Err(e) => {
                    error!("Failed to fetch details for ID {id}:\n{:?}", e);
                    // Enqueue the ID for retry
                    drop(_permit);
                    let _ = id_sender.send(ContractLocation { id, page }).await;
                    return;
                }
            };

            let contract = contract.into();
            info!("Fetched details for contract {id}");

            if let Err(e) = store.save_contract(contract, page, MAX_PAGE_SIZE).await {
                error!("Failed to save details for ID {id}:\n{:?}", e);
            }
        });

        handles.push(handle);
    }

    for handle in handles {
        let _ = handle.await;
    }
}

pub mod throttle {
    use std::sync::Arc;

    use governor::{DefaultDirectRateLimiter, Quota, RateLimiter};
    use tokio::sync::{OwnedSemaphorePermit, Semaphore};

    pub struct Throttler {
        rate_limiter: DefaultDirectRateLimiter,
        semaphore: Arc<Semaphore>,
    }

    pub struct Permit {
        _inner: OwnedSemaphorePermit,
    }

    impl Throttler {
        pub fn new(max_concurrent: usize, rate_limit_quota: Quota) -> Self {
            Throttler {
                rate_limiter: RateLimiter::direct(rate_limit_quota),
                semaphore: Arc::new(Semaphore::new(max_concurrent)),
            }
        }

        pub async fn throttle(&self) -> Permit {
            self.rate_limiter.until_ready().await;
            let permit = self.semaphore.clone().acquire_owned().await.unwrap();
            Permit { _inner: permit }
        }
    }
}
