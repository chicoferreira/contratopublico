use std::time::Duration;

use clap::Parser;
use common::{
    db::{ContractDatabase, PostgresConfig},
    searchdb::{MeilisearchConfig, SearchDatabase},
};
use log::info;
use reqwest::Url;
use scraper::sources::{
    base_api::{self, client::BaseApiClient},
    base_portal::{
        self,
        client::{BasePortalClient, ContractSort, ContractSortMethod, SortOrder},
    },
    db::RawContractDatabase,
};

#[derive(clap::Parser)]
#[command(version, about)]
struct Args {
    #[command(subcommand)]
    command: Command,
}

#[derive(clap::Subcommand)]
enum Command {
    /// Fetches a page of the Portal BASE contract list and prints it, without saving anything.
    FetchPage {
        #[arg(long, default_value = "id")]
        contract_sort_method: ContractSortMethod,
        #[arg(long, default_value = "ascending")]
        contract_sort_order: SortOrder,
        page: usize,
        #[arg(default_value_t = base_portal::scraper::MAX_PAGE_SIZE)]
        size: usize,
        base_gov_client_proxy: Option<Url>,
    },
    /// Fetches the details of a contract from Portal BASE and prints them, without saving anything.
    Fetch {
        contract_id: u64,
        base_gov_client_proxy: Option<Url>,
    },
    /// Deletes every joined contract and clears the search index, then queues every contract to be
    /// joined again. Use this to regenerate the contracts and the search index.
    Rejoin {
        #[command(flatten)]
        postgres_config: PostgresConfig,
        #[command(flatten)]
        meilisearch_config: MeilisearchConfig,
    },
    /// Downloads every contract from the BASE API into `raw_contracts`, deleting the ones it no longer has.
    SyncBaseApi {
        #[command(flatten)]
        postgres_config: PostgresConfig,
        #[arg(long, env)]
        base_api_token: String,
    },
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let env = env_logger::Env::default().filter_or("RUST_LOG", "info");
    env_logger::init_from_env(env);

    let args = Args::parse();

    match args.command {
        Command::FetchPage {
            contract_sort_method,
            contract_sort_order,
            page,
            size,
            base_gov_client_proxy,
        } => {
            let base_portal_client = BasePortalClient::new(base_gov_client_proxy);
            let sort = ContractSort {
                method: contract_sort_method,
                order: contract_sort_order,
            };
            let response = base_portal_client.fetch_page(sort, page, size).await?;

            info!("Fetched page: {response:#?}")
        }
        Command::Fetch {
            contract_id,
            base_gov_client_proxy,
        } => {
            let base_portal_client = BasePortalClient::new(base_gov_client_proxy);
            let contract = base_portal_client.get_contract_details(contract_id).await?;

            info!("Fetched contract: {contract:#?}")
        }
        Command::Rejoin {
            postgres_config,
            meilisearch_config,
        } => {
            let pool = postgres_config.connect().await?;
            let contract_db = ContractDatabase::new(pool.clone());
            let raw_contract_db = RawContractDatabase::new(pool);
            let search_db = SearchDatabase::new_from_config(meilisearch_config)?;

            let task = search_db.delete_all_contracts().await?;
            let timeout = Duration::from_mins(30);
            search_db.wait_for_success(task, timeout).await?;
            info!("Cleared the search index");

            contract_db.delete_all_contracts().await?;
            info!("Deleted every contract");

            let queued = raw_contract_db.queue_all_for_join().await?;

            info!("Queued {queued} contracts to join again")
        }
        Command::SyncBaseApi {
            postgres_config,
            base_api_token,
        } => {
            let client = BaseApiClient::new(base_api_token);
            let raw_contract_database = RawContractDatabase::new(postgres_config.connect().await?);
            base_api::scraper::sync_all(&client, &raw_contract_database).await?;
        }
    }

    Ok(())
}
