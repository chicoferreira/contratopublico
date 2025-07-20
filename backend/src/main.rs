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
use serde::{Deserialize, Serialize};
use tokio::signal;
use tracing::{Level, error, event, info};

use crate::state::{AppError, AppState};

mod state;

#[derive(Parser)]
struct Args {
    #[clap(long, env, default_value = "http://localhost:7700")]
    meilisearch_url: String,
    #[clap(long, env = "MEILI_MASTER_KEY", default_value = "masterKey")]
    meilisearch_master_key: Option<String>,
    #[clap(long, env = "BIND_URL", default_value = "0.0.0.0:3000")]
    bind_url: String,
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
        .prepare_indexes()
        .await
        .context("Failed to prepare indexes")?;

    let app = Router::new().route("/search", get(search));

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
    sort: Option<Vec<String>>,
    page: Option<usize>,
    offset: Option<usize>,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
struct SearchResponse {
    contracts: Vec<Contract>,
    total: Option<usize>,
    estimated_total: Option<usize>,
    page: usize,
    offset: usize,
}

#[tracing::instrument(skip(state))]
#[axum::debug_handler]
async fn search(
    Query(query): Query<SearchQuery>,
    State(state): State<AppState>,
) -> Result<Json<SearchResponse>, AppError> {
    let sort = query.sort.unwrap_or_default();
    let sort: Vec<&str> = sort.iter().map(|x| &**x).collect();

    let filter = query.filter.unwrap_or_default();

    let page = query.page.unwrap_or(1);
    let offset = query.offset.unwrap_or(0);

    const HITS_PER_PAGE: usize = 50;

    let results = state
        .search(&query.query, &filter, &sort, page, offset, HITS_PER_PAGE)
        .await?;

    info!("Returning {} results", results.hits.len());

    // TODO: return formatted results
    Ok(Json(SearchResponse {
        contracts: results.hits.into_iter().map(|hit| hit.result).collect(),
        page: results.page.unwrap_or(0),
        offset: results.offset.unwrap_or(0),
        total: results.total_hits,
        estimated_total: results.estimated_total_hits,
    }))
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
