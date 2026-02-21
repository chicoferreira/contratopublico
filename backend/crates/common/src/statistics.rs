use serde::{Deserialize, Serialize};

use crate::db::ContractDatabase;

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

impl ContractDatabase {
    pub async fn get_statistics(&self) -> sqlx::Result<Statistics> {
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
        .fetch_one(&self.pool)
        .await?;

        Ok(stats)
    }

    pub async fn refresh_statistics_view(&self) -> sqlx::Result<()> {
        sqlx::query!("REFRESH MATERIALIZED VIEW contract_spent_daily")
            .execute(&self.pool)
            .await?;

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use crate::{Contract, Currency};

    use super::*;
    use chrono::NaiveDate;
    use sqlx::PgPool;

    async fn insert_test_contract(
        contract_database: &ContractDatabase,
        id: u64,
        date: NaiveDate,
        price: isize,
    ) {
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

        contract_database.insert_contract(&contract).await.unwrap();
    }

    #[sqlx::test(migrations = "../../migrations")]
    async fn test_compute_statistics_empty_database(pg_pool: PgPool) {
        let db = ContractDatabase::new(pg_pool);
        db.refresh_statistics_view().await.unwrap();
        let stats = db.get_statistics().await.unwrap();

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

        let db = ContractDatabase::new(pg_pool);

        insert_test_contract(&db, 1, today - chrono::Duration::days(1), 1000).await;
        insert_test_contract(&db, 2, today - chrono::Duration::days(15), 2000).await;
        insert_test_contract(&db, 3, today - chrono::Duration::days(100), 3000).await;

        db.refresh_statistics_view().await.unwrap();
        let stats = db.get_statistics().await.unwrap();

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

        let db = ContractDatabase::new(pg_pool);

        insert_test_contract(&db, 1, today - chrono::Duration::days(400), 5000).await;
        insert_test_contract(&db, 2, today - chrono::Duration::days(1), 1000).await;

        db.refresh_statistics_view().await.unwrap();
        let stats = db.get_statistics().await.unwrap();

        assert_eq!(stats.total_spent_last_365_days, 1000);
        assert_eq!(stats.contracts_last_365_days, 1);
    }
}
