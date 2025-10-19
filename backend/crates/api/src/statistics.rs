use anyhow::Context;
use serde::{Deserialize, Serialize};
use sqlx::PgPool;
use tokio::time::Instant;
use tracing::{error, info};

use crate::state::AppState;

#[derive(Default, Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Statistics {
    pub total_spent_last_365_days: i64,
    pub contracts_last_365_days: i64,
    pub total_spent_last_30_days: i64,
    pub contracts_last_30_days: i64,
    pub total_spent_last_7_days: i64,
    pub contracts_last_7_days: i64,
}

impl Statistics {
    pub async fn get_statistics(pg_pool: &PgPool) -> anyhow::Result<Statistics> {
        let today = chrono::Local::now().date_naive();
        let before_365_days = today - chrono::Duration::days(365);
        let before_30_days = today - chrono::Duration::days(30);
        let before_7_days = today - chrono::Duration::days(7);

        let stats = sqlx::query_as!(
            Statistics,
            r#"
            WITH daily AS (
              SELECT *
              FROM contract_spent_daily
              WHERE date >= $1
            )
            SELECT
              COALESCE(SUM(amount), 0)::BIGINT AS "total_spent_last_365_days!: i64",
              COALESCE(SUM(count), 0)::BIGINT AS "contracts_last_365_days!: i64",

              COALESCE(SUM(CASE WHEN date >= $2 THEN amount END), 0)::BIGINT
                AS "total_spent_last_30_days!: i64",
              COALESCE(SUM(CASE WHEN date >= $2 THEN count END), 0)::BIGINT
                AS "contracts_last_30_days!: i64",

              COALESCE(SUM(CASE WHEN date >= $3 THEN amount END), 0)::BIGINT
                AS "total_spent_last_7_days!: i64",
              COALESCE(SUM(CASE WHEN date >= $3 THEN count END), 0)::BIGINT
                AS "contracts_last_7_days!: i64"
            FROM daily;

            "#,
            before_365_days,
            before_30_days,
            before_7_days
        )
        .fetch_one(pg_pool)
        .await?;

        Ok(stats)
    }

    pub async fn refresh_view(pg_pool: &PgPool) -> sqlx::Result<()> {
        sqlx::query!("REFRESH MATERIALIZED VIEW contract_spent_daily")
            .execute(pg_pool)
            .await?;

        Ok(())
    }
}

const STATISTICS_REFRESH_TIME: tokio::time::Duration = tokio::time::Duration::from_secs(15 * 60);

async fn reload_statistics(app_state: &AppState) -> anyhow::Result<()> {
    let instant = Instant::now();
    // currently, the materialized view is refreshed on every task execution.
    // in the future, the application will support flexible time-based queries
    // (weekly and monthly statistics) rather than the current fixed
    // intervals of 365, 30, and 7 days.
    Statistics::refresh_view(&app_state.get_pg_pool())
        .await
        .context("Failed to refresh materialized view")?;

    info!("Refreshed materialized view in {:?}", instant.elapsed());

    let instant = Instant::now();
    let statistics = Statistics::get_statistics(&app_state.get_pg_pool())
        .await
        .context("Failed to compute statistics")?;

    info!("Computed statistics in {:?}", instant.elapsed());
    app_state.set_statistics(statistics);

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

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::NaiveDate;
    use common::{Contract, Currency};

    async fn insert_test_contract(pg_pool: &PgPool, id: u64, date: NaiveDate, price: isize) {
        let contract = Contract {
            id,
            contracting_procedure_type: String::new(),
            publication_date: date,
            signing_date: Some(date),
            ccp: false,
            object_brief_description: String::new(),
            initial_contractual_price: Currency(price),
            description: None,
            contracting: Vec::new(),
            contracted: Vec::new(),
            cpvs: Vec::new(),
            regime: None,
            contract_status: None,
            non_written_contract_justification_types: String::new(),
            contract_types: String::new(),
            execution_deadline_days: 0,
            execution_place: String::new(),
            contract_fundamentation_type: String::new(),
            contestants: Vec::new(),
            invitees: Vec::new(),
            documents: Vec::new(),
            contracting_procedure_url: None,
            announcement_id: None,
            direct_award_fundamentation_type: String::new(),
            observations: None,
            end_of_contract_type: None,
            close_date: None,
            total_effective_price: None,
            causes_deadline_change: None,
            causes_price_change: None,
        };

        common::db::insert_contract(&contract, pg_pool)
            .await
            .unwrap();
    }

    #[sqlx::test(migrations = "../../migrations")]
    async fn test_compute_statistics_empty_database(pg_pool: PgPool) {
        Statistics::refresh_view(&pg_pool).await.unwrap();
        let stats = Statistics::get_statistics(&pg_pool).await.unwrap();

        assert_eq!(stats.total_spent_last_365_days, 0);
        assert_eq!(stats.contracts_last_365_days, 0);
        assert_eq!(stats.total_spent_last_30_days, 0);
        assert_eq!(stats.contracts_last_30_days, 0);
        assert_eq!(stats.total_spent_last_7_days, 0);
        assert_eq!(stats.contracts_last_7_days, 0);
    }

    #[sqlx::test(migrations = "../../migrations")]
    async fn test_compute_statistics_with_recent_contracts(pg_pool: PgPool) {
        let today = chrono::Local::now().date_naive();

        insert_test_contract(&pg_pool, 1, today - chrono::Duration::days(1), 1000).await;
        insert_test_contract(&pg_pool, 2, today - chrono::Duration::days(15), 2000).await;
        insert_test_contract(&pg_pool, 3, today - chrono::Duration::days(100), 3000).await;

        Statistics::refresh_view(&pg_pool).await.unwrap();
        let stats = Statistics::get_statistics(&pg_pool).await.unwrap();

        assert_eq!(stats.total_spent_last_365_days, 6000);
        assert_eq!(stats.contracts_last_365_days, 3);
        assert_eq!(stats.total_spent_last_30_days, 3000);
        assert_eq!(stats.contracts_last_30_days, 2);
        assert_eq!(stats.total_spent_last_7_days, 1000);
        assert_eq!(stats.contracts_last_7_days, 1);
    }

    #[sqlx::test(migrations = "../../migrations")]
    async fn test_compute_statistics_old_contracts_excluded(pg_pool: PgPool) {
        let today = chrono::Local::now().date_naive();

        insert_test_contract(&pg_pool, 1, today - chrono::Duration::days(400), 5000).await;
        insert_test_contract(&pg_pool, 2, today - chrono::Duration::days(1), 1000).await;

        Statistics::refresh_view(&pg_pool).await.unwrap();
        let stats = Statistics::get_statistics(&pg_pool).await.unwrap();

        assert_eq!(stats.total_spent_last_365_days, 1000);
        assert_eq!(stats.contracts_last_365_days, 1);
    }
}
