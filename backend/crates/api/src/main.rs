use crate::state::AppState;
use anyhow::Context;
use clap::Parser;
use meilisearch_sdk::client::Client;
use reqwest::Url;
use scraper::{base_gov::client::BaseGovClient, store::Store};
use std::{path::PathBuf, sync::Arc, time::Instant};
use tokio::signal;
use tracing::{Level, error, event, info};

mod error;
mod extractors;
mod filter;
mod metrics;
mod router;
mod sort;
mod state;
mod statistics;

#[derive(Parser)]
struct Args {
    #[clap(long, env, default_value = "http://localhost:7700")]
    meilisearch_url: String,
    #[clap(long, env = "MEILI_MASTER_KEY", default_value = "masterKey")]
    meilisearch_master_key: Option<String>,
    #[clap(long, env, default_value = "0.0.0.0:3000")]
    bind_url: String,
    #[clap(long, env, default_value = "0.0.0.0:3001")]
    metrics_bind_url: String,
    #[clap(long, env, default_value = "60")]
    scraper_interval_secs: u64,
    #[clap(long, env, default_value = "../data/scraper/saved_pages.json")]
    saved_pages_path: PathBuf,
    #[clap(long, env, default_value = "localhost")]
    postgres_host: String,
    #[clap(long, env, default_value = "5432")]
    postgres_port: u16,
    #[clap(long, env, default_value = "contratopublico")]
    postgres_user: String,
    #[clap(long, env, default_value = "contratopublico")]
    postgres_password: String,
    #[clap(long, env, default_value = "contratopublico")]
    postgres_db: String,
    #[clap(long, env)]
    no_scraper: bool,
    #[clap(long, env)]
    base_gov_client_proxy: Option<Url>,
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt::fmt().init();

    let args = Args::parse();
    let meilisearch = Client::new(
        &args.meilisearch_url,
        args.meilisearch_master_key.as_deref(),
    )
    .context("Failed to create Meilisearch client")?;

    let pg_options = sqlx::postgres::PgConnectOptions::new()
        .host(&args.postgres_host)
        .port(args.postgres_port)
        .username(&args.postgres_user)
        .password(&args.postgres_password)
        .database(&args.postgres_db);

    let pg_pool = sqlx::postgres::PgPool::connect_with(pg_options)
        .await
        .context("Failed to connect to PostgreSQL")?;

    sqlx::migrate!("../../migrations")
        .run(&pg_pool)
        .await
        .context("Failed to run migrations")?;

    let scraper_store = Arc::new(
        Store::new(meilisearch.clone(), pg_pool.clone(), args.saved_pages_path)
            .context("Failed to create scraper store")?,
    );

    let app_state = AppState::new(meilisearch, pg_pool);
    app_state
        .prepare_settings()
        .await
        .context("Failed to prepare indexes")?;

    if !args.no_scraper {
        tokio::spawn(async move {
            loop {
                let base_gov_client = BaseGovClient::new(args.base_gov_client_proxy.clone());
                scraper::scraper::scrape(scraper_store.clone(), base_gov_client).await;
                tokio::time::sleep(tokio::time::Duration::from_secs(args.scraper_interval_secs))
                    .await;
            }
        });
    }

    let app_state_clone = app_state.clone();
    tokio::spawn(async move {
        let client = app_state_clone.get_client();
        loop {
            let instant = Instant::now();
            // this will likely change soon to use a more efficient database with aggregation mechanisms
            match statistics::Statistics::compute_new_statistics(client.clone()).await {
                Ok(statistics) => {
                    info!("Computed statistics in {:?}", instant.elapsed());

                    app_state_clone.set_statistics(statistics);
                }
                Err(err) => {
                    error!("Failed to compute statistics: {:?}", err);
                }
            }

            const STATISTICS_REFRESH_TIME: tokio::time::Duration =
                tokio::time::Duration::from_secs(15 * 60);
            tokio::time::sleep(STATISTICS_REFRESH_TIME).await;
        }
    });

    let backend_router = router::router(app_state);

    let backend_listener = tokio::net::TcpListener::bind(args.bind_url)
        .await
        .context("Failed to bind backend listener")?;

    let backend_ip = backend_listener.local_addr().unwrap();
    event!(Level::INFO, "Backend listening on {backend_ip}");

    let metrics_router = metrics::metrics_router()?;

    let metrics_listener = tokio::net::TcpListener::bind(args.metrics_bind_url)
        .await
        .context("Failed to bind metrics listener")?;

    let metrics_ip = metrics_listener.local_addr().unwrap();
    event!(Level::INFO, "Metrics listening on {metrics_ip}");

    let (metrics_task, backend_task) = tokio::join!(
        axum::serve(metrics_listener, metrics_router)
            .with_graceful_shutdown(shutdown_signal("metrics")),
        axum::serve(backend_listener, backend_router)
            .with_graceful_shutdown(shutdown_signal("backend")),
    );

    metrics_task.context("Failed to serve metrics")?;
    backend_task.context("Failed to serve backend")
}

async fn shutdown_signal(target: &str) {
    let ctrl_c = async {
        signal::ctrl_c()
            .await
            .expect("failed to install Ctrl+C handler");
    };

    #[cfg(unix)]
    let terminate = async {
        signal::unix::signal(signal::unix::SignalKind::terminate())
            .expect("failed to install signal handler")
            .recv()
            .await;
    };

    #[cfg(not(unix))]
    let terminate = std::future::pending::<()>();

    tokio::select! {
        _ = ctrl_c => {},
        _ = terminate => {},
    }

    info!("Shutting down {target}...");
}
