use std::path::PathBuf;

use anyhow::Context;
use axum::{
    Json, Router,
    extract::{Query, State},
    http::{Response, StatusCode},
    response::IntoResponse,
    routing::get,
};
use clap::Parser;
use common::Contract;
use meilisearch_sdk::client::Client;
use scraper::store::meilisearch::MeilisearchStore;
use serde::{Deserialize, Serialize};
use tokio::signal;
use tracing::{Level, error, event, info};

use crate::{
    sort::SortBy,
    state::{AppError, AppState},
};

mod sort;
mod state;

#[derive(Parser)]
struct Args {
    #[clap(long, env, default_value = "http://localhost:7700")]
    meilisearch_url: String,
    #[clap(long, env = "MEILI_MASTER_KEY", default_value = "masterKey")]
    meilisearch_master_key: Option<String>,
    #[clap(long, env, default_value = "0.0.0.0:3000")]
    bind_url: String,
    #[clap(long, env, default_value = "60")]
    scraper_interval_secs: u64,
    #[clap(long, env, default_value = "data/scraper/saved_pages.json")]
    saved_pages_path: PathBuf,
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

    let app_state = AppState::new(meilisearch);
    app_state
        .prepare_settings()
        .await
        .context("Failed to prepare indexes")?;

    let meilisearch = app_state.get_client();

    let scraper_store = MeilisearchStore::new(meilisearch, args.saved_pages_path)
        .context("Failed to create scraper store")?;

    tokio::spawn(async move {
        loop {
            scraper::scraper::scrape(scraper_store.clone()).await;
            tokio::time::sleep(tokio::time::Duration::from_secs(args.scraper_interval_secs)).await;
        }
    });

    let app = Router::new().route("/api/search", get(search));

    let listener = tokio::net::TcpListener::bind(args.bind_url)
        .await
        .context("Failed to bind listener")?;

    event!(
        Level::INFO,
        "Listening on {}",
        listener.local_addr().unwrap()
    );

    axum::serve(listener, app.with_state(app_state))
        .with_graceful_shutdown(shutdown_signal())
        .await
        .context("Failed to serve axum")
}

#[derive(Debug, Deserialize)]
struct SearchQuery {
    query: String,
    filter: Option<String>,
    #[serde(flatten)]
    sort: Option<SortBy>,
    page: Option<usize>,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
struct SearchResponse {
    contracts: Vec<Contract>,
    total: usize,
    page: usize,
    total_pages: usize,
    elapsed_millis: u64,
    hits_per_page: usize,
}

#[tracing::instrument(skip(state))]
#[axum::debug_handler]
async fn search(
    Query(query): Query<SearchQuery>,
    State(state): State<AppState>,
) -> Result<Json<SearchResponse>, AppError> {
    let sort = query.sort.unwrap_or_default();
    let sort: Vec<&str> = vec![sort.to_meilisearch()];

    let filter = query.filter.unwrap_or_default();

    let page = query.page.unwrap_or(1);

    const HITS_PER_PAGE: usize = 20;

    let response = state
        .search(&query.query, &filter, &sort, page, HITS_PER_PAGE)
        .await?;

    info!("Returning {} results", response.contracts.len());

    // TODO: return formatted results
    Ok(Json(response))
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response<axum::body::Body> {
        match self {
            AppError::MeilisearchError(e) => {
                error!("Meilisearch error: {:?}", e);
                StatusCode::INTERNAL_SERVER_ERROR.into_response()
            }
        }
    }
}

async fn shutdown_signal() {
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

    info!("Shutting down...");
}
