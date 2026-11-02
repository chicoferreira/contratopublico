use std::path::PathBuf;

use clap::Parser;
use common::{
    db::{ContractDatabase, PostgresConfig},
    searchdb::{MeilisearchConfig, SearchDatabase},
};
use log::info;
use reqwest::Url;
use scraper::{
    export, search,
    sources::base_portal::{
        self,
        client::{BasePortalClient, ContractSort, ContractSortMethod, SortOrder},
    },
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
    ExportOldFormatToJson {
        #[command(flatten)]
        meilisearch_config: MeilisearchConfig,
        output_path: PathBuf,
    },
    RebuildSearchIndex {
        #[command(flatten)]
        postgres_config: PostgresConfig,
        #[command(flatten)]
        meilisearch_config: MeilisearchConfig,
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
        Command::ExportOldFormatToJson {
            meilisearch_config,
            output_path,
        } => {
            let meilisearch_client = meilisearch_config.create_client()?;
            export::export_old_format_to_json(meilisearch_client, output_path).await?;
        }
        Command::RebuildSearchIndex {
            postgres_config,
            meilisearch_config,
        } => {
            let contract_database = ContractDatabase::new(postgres_config.connect().await?);
            let search_database = SearchDatabase::new(meilisearch_config.create_client()?);
            tokio::select! {
                result = search::rebuild::rebuild_search_index(&contract_database, &search_database) => result?,
                _ = tokio::signal::ctrl_c() => {
                    info!("Rebuild search index interrupted by user");
                },
            }
        }
    }

    Ok(())
}
