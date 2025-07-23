use std::{path::PathBuf, sync::Arc};

use anyhow::Context;
use clap::Parser;
use scraper::importer;

#[derive(clap::Parser)]
#[command(version, about)]
struct Args {
    #[command(subcommand)]
    command: Command,
}

#[derive(clap::Subcommand)]
enum Command {
    Scrape {
        #[command(subcommand)]
        store: StoreCommand,
    },
    Import {
        meilisearch_url: String,
        meilisearch_api_key: Option<String>,
    },
}

#[derive(Clone, clap::Subcommand)]
enum StoreCommand {
    File,
    Meilisearch {
        url: String,
        api_key: Option<String>,
        saved_pages_path: PathBuf,
    },
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let env = env_logger::Env::default().filter_or("RUST_LOG", "info");
    env_logger::init_from_env(env);

    let args = Args::parse();

    match args.command {
        Command::Scrape { store } => match store {
            StoreCommand::File => {
                scraper::scraper::scrape(scraper::store::file::FileStore).await;
            }
            StoreCommand::Meilisearch {
                url,
                api_key,
                saved_pages_path,
            } => {
                let client = meilisearch_sdk::client::Client::new(url, api_key)
                    .context("Failed to create Meilisearch client")?;

                let store = scraper::store::meilisearch::MeilisearchStore::new(
                    Arc::new(client),
                    saved_pages_path,
                )
                .context("Failed to create MeilisearchStore")?;

                scraper::scraper::scrape(store).await;
            }
        },
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
