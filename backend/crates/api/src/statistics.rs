use anyhow::Context;
use tokio::time::Instant;
use tracing::{error, info};

use crate::state::AppState;

const STATISTICS_REFRESH_TIME: tokio::time::Duration = tokio::time::Duration::from_secs(15 * 60);

async fn reload_statistics(app_state: &AppState) -> anyhow::Result<()> {
    let instant = Instant::now();
    // currently, the materialized view is refreshed on every task execution.
    // in the future, the application will support flexible time-based queries
    // (weekly and monthly statistics) rather than the current fixed
    // intervals of 365, 30, and 7 days.

    app_state
        .reload_statistics()
        .await
        .context("Failed to compute statistics")?;

    info!(
        "Refreshed materialized view and computed statistics in {:?}",
        instant.elapsed()
    );

    Ok(())
}

pub async fn run_reload_statistics_task(app_state: AppState) -> anyhow::Result<()> {
    loop {
        match reload_statistics(&app_state).await {
            Ok(_) => {}
            Err(err) => error!("Failed to reload statistics: {:?}", err),
        }

        tokio::time::sleep(STATISTICS_REFRESH_TIME).await;
    }
}
