use governor::{Quota, RateLimiter};
use log::{error, info};
use std::{
    num::NonZeroU32,
    sync::{Arc, Mutex},
};
use tokio::sync::Semaphore;

use crate::{base_gov::ContractSort, store::save_contracts_page_to_file};

pub mod base_gov;
mod store;

const MAX_PAGE_SIZE: usize = 50;
const CONTRACT_SORT_ORDER: ContractSort = base_gov::ContractSort {
    method: base_gov::ContractSortMethod::Id,
    order: base_gov::SortOrder::Ascending,
};
const MAX_CONCURRENT_REQUESTS: usize = 3;
const MAX_REQUEST_QUOTA: Quota = Quota::per_minute(NonZeroU32::new(100).unwrap());

#[tokio::main]
async fn main() {
    let env = env_logger::Env::default().filter_or("RUST_LOG", "info");
    env_logger::init_from_env(env);

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
        if store::is_page_completed(current_page, MAX_PAGE_SIZE) {
            info!("Page {current_page} is already completed, skipping...");
            current_page += 1;
            continue;
        }

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
        let task = tokio::spawn(async move {
            let _permit = permit; // hold permit until task ends

            let response = match client.search_contracts(payload).await {
                Ok(response) => response,
                Err(e) => {
                    error!("Failed to fetch page {current_page}: {}", e);
                    return;
                }
            };

            info!(
                "Fetched page {current_page} with {} contracts. Saving to disk...",
                response.items.len()
            );

            if let Ok(mut total_pages) = total_pages.lock() {
                let new_total_pages = response.total / MAX_PAGE_SIZE + 1;
                if total_pages.is_none_or(|total_pages| total_pages < new_total_pages) {
                    *total_pages = Some(new_total_pages);
                }
            }

            if let Err(e) = save_contracts_page_to_file(&response.items, current_page) {
                error!("Failed to save contracts page {current_page}: {}", e);
            }
        });

        handles.push(task);

        current_page += 1;
    }

    for handle in handles {
        handle.await.unwrap();
    }
}
