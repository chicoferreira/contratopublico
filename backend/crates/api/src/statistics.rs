use std::sync::Arc;

use crate::filter::Filters;
use anyhow::Context;
use chrono::{Local, NaiveDate};
use meilisearch_sdk::documents::DocumentsQuery;
use serde::{Deserialize, Serialize};

#[derive(Default, Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Statistics {
    pub total_spent_last_365_days: u64,
    pub contracts_last_365_days: u64,
    pub total_spent_last_30_days: u64,
    pub contracts_last_30_days: u64,
    pub total_spent_last_7_days: u64,
    pub contracts_last_7_days: u64,
}

impl Statistics {
    pub async fn compute_new_statistics(
        client: Arc<meilisearch_sdk::client::Client>,
    ) -> anyhow::Result<Statistics> {
        // this is fast (500ms every 10 minutes) for now, will change to postgres later for day-to-day spending graphs

        let today = Local::now().date_naive();
        let before_365_days = today - chrono::Duration::days(365);
        let before_30_days = today - chrono::Duration::days(30);
        let before_7_days = today - chrono::Duration::days(7);

        let filter = Filters {
            start_publication_date: Some(before_365_days),
            end_publication_date: Some(today),
            ..Default::default()
        }
        .to_meilisearch()
        .join(" AND ");

        let index = client.index("contracts");

        #[derive(Debug, Deserialize, Serialize, Clone, PartialEq, Eq)]
        #[serde(rename_all = "camelCase")]
        struct Price {
            initial_contractual_price: i64,
            publication_date: NaiveDate,
        }

        let prices = DocumentsQuery::new(&index)
            .with_fields(["initialContractualPrice", "publicationDate"])
            .with_filter(&filter)
            .with_limit(u32::MAX as usize)
            .execute::<Price>()
            .await
            .context("Couldn't grab documents on updating statistics")?
            .results;

        let mut total_spent_last_365_days = 0;
        let contracts_last_365_days = prices.len() as u64;
        let mut total_spent_last_30_days = 0;
        let mut contracts_last_30_days = 0;
        let mut total_spent_last_7_days = 0;
        let mut contracts_last_7_days = 0;

        for price in prices {
            if price.initial_contractual_price < 0 {
                continue;
            }

            total_spent_last_365_days += price.initial_contractual_price as u64;

            if price.publication_date >= before_30_days {
                total_spent_last_30_days += price.initial_contractual_price as u64;
                contracts_last_30_days += 1;
            }

            if price.publication_date >= before_7_days {
                total_spent_last_7_days += price.initial_contractual_price as u64;
                contracts_last_7_days += 1;
            }
        }

        Ok(Statistics {
            total_spent_last_365_days,
            contracts_last_365_days,
            total_spent_last_30_days,
            contracts_last_30_days,
            total_spent_last_7_days,
            contracts_last_7_days,
        })
    }
}
