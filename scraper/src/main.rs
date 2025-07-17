use anyhow::Context;
use clap::Parser;

pub mod base_gov;
mod importer;
mod scraper;
mod store;

#[derive(clap::Parser)]
#[command(version, about)]
struct Args {
    #[command(subcommand)]
    command: Command,
}

#[derive(clap::Subcommand)]
enum Command {
    Scrape,
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
        Command::Scrape => {
            scraper::scrape().await;
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
