use std::{
    io::{self, BufReader},
    time::Duration,
};

use anyhow::Context;
use futures::TryStreamExt;
use serde::{Serialize, de::DeserializeOwned};
use tokio_util::io::{StreamReader, SyncIoBridge};

use crate::sources::base_api::{BaseApiContract, BaseApiModification};

const CONTRACTS_URL: &str = "https://www.base.gov.pt/APIBase2/GetInfoContrato";
const MODIFICATIONS_URL: &str = "https://www.base.gov.pt/APIBase2/GetInfoModContrat";
const USER_AGENT: &str = "contratopublico.pt";
const TIMEOUT: Duration = Duration::from_secs(600);

pub struct BaseApiClient {
    client: reqwest::Client,
    token: String,
}

impl BaseApiClient {
    pub fn new(token: String) -> Self {
        let client = reqwest::Client::builder()
            .user_agent(USER_AGENT)
            .timeout(TIMEOUT)
            .build()
            .unwrap();

        Self { client, token }
    }

    /// Fetches every contract of a year.
    pub async fn fetch_by_year(&self, year: u16) -> anyhow::Result<Vec<BaseApiContract>> {
        self.fetch(CONTRACTS_URL, &[("Ano", year)]).await
    }

    /// Fetches the contracts whose last update was in the last `days` days, up to 90.
    pub async fn fetch_by_previous_days(&self, days: u16) -> anyhow::Result<Vec<BaseApiContract>> {
        self.fetch(CONTRACTS_URL, &[("numDias", days)]).await
    }

    /// Fetches the modifications to the contracts of a year.
    /// This endpoint can't filter by previous days.
    pub async fn fetch_modifications_by_year(
        &self,
        year: u16,
    ) -> anyhow::Result<Vec<BaseApiModification>> {
        self.fetch(MODIFICATIONS_URL, &[("Ano", year)]).await
    }

    async fn fetch<T, Q>(&self, url: &str, query: &Q) -> anyhow::Result<Vec<T>>
    where
        T: DeserializeOwned + Send + 'static,
        Q: Serialize + ?Sized,
    {
        let response = self
            .client
            .get(url)
            .query(query)
            .header("_AcessToken", &self.token) // yes, it's misspelled in the API
            .send()
            .await
            .context("Failed to send API request")?
            .error_for_status()
            .context("API returned an error status")?;

        let body = response.bytes_stream().map_err(io::Error::other);
        let reader = BufReader::new(SyncIoBridge::new(Box::pin(StreamReader::new(body))));

        // serde_json reads synchronously, so it can't run on the async runtime
        tokio::task::spawn_blocking(move || serde_json::from_reader(reader))
            .await
            .context("API response parser panicked")?
            // the API answers an invalid token with status 200 and plain text
            .context("Failed to parse API response, check that the token is valid")
    }
}
