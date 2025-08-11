use std::{
    num::NonZeroU32,
    sync::{Arc, Mutex},
};

use governor::{Quota, RateLimiter};
use log::{error, info};
use tokio::sync::Semaphore;

use crate::{
    base_gov::{self, ContractSort},
    store::{self},
};

const MAX_PAGE_SIZE: usize = 50;
const CONTRACT_SORT_ORDER: ContractSort = base_gov::ContractSort {
    method: base_gov::ContractSortMethod::Id,
    order: base_gov::SortOrder::Ascending,
};
// TODO: make this adaptive based on the availability of base gov
const MAX_CONCURRENT_REQUESTS: usize = 1;
const MAX_REQUEST_QUOTA: Quota = Quota::per_minute(NonZeroU32::new(100).unwrap());

// Max consecutive failures when the API keeps failing since the start making it so we don't know the number of pages to scrape
const MAX_CONSECUTIVE_FAILURES: usize = 10;

pub async fn scrape(store: impl store::Store + 'static) {
    let client = Arc::new(base_gov::BaseGovClient::new());

    let total_pages = Arc::new(Mutex::new(None));
    let consecutive_failures = Arc::new(Mutex::new(0_usize));
    let mut current_page = 0_usize;

    let limiter = Arc::new(RateLimiter::direct(MAX_REQUEST_QUOTA));
    let semaphore = Arc::new(Semaphore::new(MAX_CONCURRENT_REQUESTS));

    let mut handles = vec![];

    loop {
        limiter.until_ready().await;
        let permit = semaphore.clone().acquire_owned().await.unwrap();

        let should_continue = match *total_pages.lock().unwrap() {
            Some(total_pages) => current_page < total_pages,
            None => *consecutive_failures.lock().unwrap() < MAX_CONSECUTIVE_FAILURES,
        };

        if !should_continue {
            break;
        }

        current_page = store.get_next_page_to_query(current_page);

        let total_pages_str = total_pages
            .lock()
            .unwrap()
            .map(|s| s.to_string())
            .unwrap_or("?".to_string());

        info!("Fetching page {current_page}/{total_pages_str}...");

        let payload = base_gov::BaseGovPayload::new_search_all_contracts(
            CONTRACT_SORT_ORDER,
            current_page,
            MAX_PAGE_SIZE,
        );

        let client = Arc::clone(&client);
        let total_pages = Arc::clone(&total_pages);
        let consecutive_failures = Arc::clone(&consecutive_failures);

        let store = store.clone();

        let task = tokio::spawn(async move {
            let _permit = permit; // hold permit until task ends

            let response = match client.search_contracts(payload).await {
                Ok(response) => response,
                Err(e) => {
                    *consecutive_failures.lock().unwrap() += 1;
                    error!("Failed to fetch page {current_page}:\n{:?}", e);
                    return;
                }
            };

            info!(
                "Fetched page {current_page} with {} contracts...",
                response.items.len()
            );

            if let Ok(mut total_pages) = total_pages.lock() {
                // use ceil division to ensure the last (incomplete) page is accounted for
                let new_total_pages = (response.total + MAX_PAGE_SIZE - 1) / MAX_PAGE_SIZE;
                if total_pages.is_none_or(|total_pages| total_pages < new_total_pages) {
                    *total_pages = Some(new_total_pages);
                }
            }

            let contracts: Vec<_> = response.items.into_iter().map(Into::into).collect();

            if let Err(e) = store
                .save_contracts_page(&contracts, current_page, MAX_PAGE_SIZE)
                .await
            {
                error!("Failed to save contracts page {current_page}:\n{:?}", e);
            }
        });

        handles.push(task);

        current_page += 1;
    }

    for handle in handles {
        handle.await.unwrap();
    }
}
