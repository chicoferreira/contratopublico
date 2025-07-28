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

pub async fn scrape(store: impl store::Store + 'static) {
    let client = Arc::new(base_gov::BaseGovClient::new());

    let total_pages = Arc::new(Mutex::new(None));
    let mut current_page = 0_usize;

    let limiter = Arc::new(RateLimiter::direct(MAX_REQUEST_QUOTA));
    let semaphore = Arc::new(Semaphore::new(MAX_CONCURRENT_REQUESTS));

    let mut handles = vec![];

    while total_pages
        .lock()
        .unwrap()
        .is_none_or(|total_pages| current_page < total_pages)
    {
        current_page = store.get_next_page_to_query(current_page);

        limiter.until_ready().await;

        let permit = semaphore.clone().acquire_owned().await.unwrap();

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

        let store = store.clone();

        let task = tokio::spawn(async move {
            let _permit = permit; // hold permit until task ends

            let response = match client.search_contracts(payload).await {
                Ok(response) => response,
                Err(e) => {
                    error!("Failed to fetch page {current_page}:\n{:?}", e);
                    return;
                }
            };

            info!(
                "Fetched page {current_page} with {} contracts...",
                response.items.len()
            );

            if let Ok(mut total_pages) = total_pages.lock() {
                let new_total_pages = response.total / MAX_PAGE_SIZE + 1;
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
