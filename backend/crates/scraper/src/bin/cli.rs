use clap::Parser;
use common::db::PostgresConfig;
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
    Fetch {
        contract_id: u64,
        base_gov_client_proxy: Option<Url>,
    },
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
