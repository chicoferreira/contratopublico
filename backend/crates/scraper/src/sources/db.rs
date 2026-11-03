use serde::{Serialize, de::DeserializeOwned};
use sqlx::{PgPool, postgres::PgListener, types::Json};

use crate::sources::ContractSource;

#[derive(Debug, Clone)]
pub struct RawContractDatabase {
    pool: PgPool,
}

#[derive(Debug, Clone, Copy, sqlx::FromRow)]
pub struct QueueEntry {
    pub contract_id: i64,
    pub version: i64,
}

impl RawContractDatabase {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }

    pub async fn save_contracts<T: Serialize + Sync>(
        &self,
        source: ContractSource,
        contracts: &[(u64, T)],
    ) -> Result<(), sqlx::Error> {
        let (ids, data): (Vec<i64>, Vec<Json<&T>>) = contracts
            .iter()
            .map(|(id, contract)| (*id as i64, Json(contract)))
            .unzip();

        sqlx::query(
            r#"
            INSERT INTO raw_contracts (source, contract_id, data)
            SELECT $1, * FROM UNNEST($2::BIGINT[], $3::JSONB[])
            ON CONFLICT (source, contract_id) DO UPDATE
            SET data = EXCLUDED.data, fetched_at = now()
            WHERE raw_contracts.data IS DISTINCT FROM EXCLUDED.data
            "#,
        )
        .bind(source)
        .bind(ids)
        .bind(data)
        .execute(&self.pool)
        .await?;

        Ok(())
    }

    pub async fn get_contract<T: DeserializeOwned + Send + Unpin + 'static>(
        &self,
        source: ContractSource,
        id: i64,
    ) -> Result<Option<T>, sqlx::Error> {
        let contract = sqlx::query_scalar::<_, Json<T>>(
            "SELECT data FROM raw_contracts WHERE source = $1 AND contract_id = $2",
        )
        .bind(source)
        .bind(id)
        .fetch_optional(&self.pool)
        .await?;

        Ok(contract.map(|contract| contract.0))
    }

    pub async fn list_queued_after(
        &self,
        last_id: i64,
        limit: i64,
    ) -> Result<Vec<QueueEntry>, sqlx::Error> {
        sqlx::query_as(
            r#"
            SELECT contract_id, version
            FROM join_queue
            WHERE contract_id > $1
            ORDER BY contract_id
            LIMIT $2
            "#,
        )
        .bind(last_id)
        .bind(limit)
        .fetch_all(&self.pool)
        .await
    }

    pub async fn dequeue(&self, queued: &[QueueEntry]) -> Result<(), sqlx::Error> {
        let (ids, versions): (Vec<i64>, Vec<i64>) = queued
            .iter()
            .map(|entry| (entry.contract_id, entry.version))
            .unzip();

        sqlx::query(
            r#"
            DELETE FROM join_queue q
            USING UNNEST($1::BIGINT[], $2::BIGINT[]) AS b(contract_id, version)
            WHERE q.contract_id = b.contract_id AND q.version = b.version
            "#,
        )
        .bind(ids)
        .bind(versions)
        .execute(&self.pool)
        .await?;

        Ok(())
    }

    /// Returns the queued contract with the lowest id, skipping `excluded_ids`.
    pub async fn next_portal_refetch(
        &self,
        excluded_ids: &[i64],
    ) -> Result<Option<QueueEntry>, sqlx::Error> {
        sqlx::query_as(
            r#"
            SELECT contract_id, version
            FROM portal_refetch_queue
            WHERE contract_id <> ALL($1)
            ORDER BY contract_id
            LIMIT 1
            "#,
        )
        .bind(excluded_ids)
        .fetch_optional(&self.pool)
        .await
    }

    /// Returns a listener that is notified every time contracts are added to the queue,
    /// which happens when a BASE API fetch changes a saved contract.
    pub async fn listen_portal_refetches(&self) -> Result<PgListener, sqlx::Error> {
        let mut listener = PgListener::connect_with(&self.pool).await?;
        listener.listen("portal_refetch_queue").await?;

        Ok(listener)
    }

    /// Removes the contract from the queue, unless it was queued again since `entry` was read (i.e. the version is different).
    pub async fn dequeue_portal_refetch(&self, entry: QueueEntry) -> Result<(), sqlx::Error> {
        sqlx::query("DELETE FROM portal_refetch_queue WHERE contract_id = $1 AND version = $2")
            .bind(entry.contract_id)
            .bind(entry.version)
            .execute(&self.pool)
            .await?;

        Ok(())
    }
}
