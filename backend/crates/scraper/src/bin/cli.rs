use std::{path::PathBuf, sync::Arc};

use anyhow::Context;
use clap::Parser;
use common::Contract;
use log::info;
use reqwest::Url;
use scraper::{
    base_gov::client::BaseGovClient,
    config::{MeilisearchConfig, PostgresConfig},
    export, migrate,
};

#[derive(clap::Parser)]
#[command(version, about)]
struct Args {
    #[command(subcommand)]
    command: Command,
}

#[derive(clap::Subcommand)]
enum Command {
    Scrape {
        #[command(flatten)]
        postgres_config: PostgresConfig,
        #[command(flatten)]
        meilisearch_config: MeilisearchConfig,
        saved_pages_path: PathBuf,
        base_gov_client_proxy: Option<Url>,
    },
    Fetch {
        contract_id: u64,
        base_gov_client_proxy: Option<Url>,
    },
    ExportOldFormatToJson {
        #[command(flatten)]
        meilisearch_config: MeilisearchConfig,
        output_path: PathBuf,
    },
    MigrateToPostgres {
        #[command(flatten)]
        postgres_config: PostgresConfig,
        #[command(flatten)]
        meilisearch_config: MeilisearchConfig,
        contracts_path: PathBuf,
    },
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let env = env_logger::Env::default().filter_or("RUST_LOG", "info");
    env_logger::init_from_env(env);

    let args = Args::parse();

    match args.command {
        Command::Scrape {
            saved_pages_path,
            base_gov_client_proxy,
            postgres_config,
            meilisearch_config,
        } => {
            let meili_client = meilisearch_config.create_client()?;
            let pg_pool = postgres_config.create_pool().await?;

            let store = scraper::store::Store::new(meili_client, pg_pool, saved_pages_path)
                .context("Failed to create store")?;

            let base_gov_client = BaseGovClient::new(base_gov_client_proxy);
            scraper::scraper::scrape(Arc::new(store), base_gov_client).await;
        }
        Command::Fetch {
            contract_id,
            base_gov_client_proxy,
        } => {
            let base_gov_client = BaseGovClient::new(base_gov_client_proxy);
            let contract = base_gov_client.get_contract_details(contract_id).await?;
            let contract: Contract = contract.into();

            info!("Fetched contract: {contract:#?}")
        }
        Command::MigrateToPostgres {
            postgres_config,
            meilisearch_config,
            contracts_path,
        } => {
            let meili_client = meilisearch_config.create_client()?;
            let pg_pool = postgres_config.create_pool().await?;

            migrate::migrate_contracts_to_postgres(contracts_path, meili_client, pg_pool)
                .await
                .context("Failed to import contracts")?;
        }
        Command::ExportOldFormatToJson {
            meilisearch_config,
            output_path,
        } => {
            let meili_client = meilisearch_config.create_client()?;

            export::export_old_format_to_json(meili_client, output_path).await?;
        }
    }

    Ok(())
}
