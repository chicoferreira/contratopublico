use std::{path::PathBuf, sync::Arc};

use anyhow::Context;
use clap::Parser;
use reqwest::Url;
use scraper::{base_gov::client::BaseGovClient, importer};

#[derive(clap::Parser)]
#[command(version, about)]
struct Args {
    #[command(subcommand)]
    command: Command,
}

#[derive(clap::Subcommand)]
enum Command {
    Scrape {
        saved_pages_path: PathBuf,
        url: String,
        api_key: Option<String>,
        base_gov_client_proxy: Option<Url>,
    },
    Import {
        meilisearch_url: String,
        meilisearch_api_key: Option<String>,
    },
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let env = env_logger::Env::default().filter_or("RUST_LOG", "info");
    env_logger::init_from_env(env);

    let args = Args::parse();

    match args.command {
        Command::Scrape {
            url,
            api_key,
            saved_pages_path,
            base_gov_client_proxy,
        } => {
            let client = meilisearch_sdk::client::Client::new(url, api_key)
                .context("Failed to create Meilisearch client")?;

            let store = scraper::store::Store::new(client, saved_pages_path)
                .context("Failed to create store")?;

            let base_gov_client = BaseGovClient::new(base_gov_client_proxy);

            scraper::scraper::scrape(Arc::new(store), base_gov_client).await;
        }
        Command::Import {
            meilisearch_url,
            meilisearch_api_key,
        } => {
            importer::import_contracts_to_meilisearch(meilisearch_url, meilisearch_api_key)
                .await
                .context("Failed to import contracts")?;
        }
    }

    Ok(())
}
