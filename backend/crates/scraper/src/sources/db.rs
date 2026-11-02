use serde::{Serialize, de::DeserializeOwned};
use sqlx::{PgPool, types::Json};

use crate::sources::ContractSource;

#[derive(Debug, Clone)]
pub struct RawContractDatabase {
    pool: PgPool,
}

#[derive(Debug, Clone, Copy, sqlx::FromRow)]
pub struct JoinQueueEntry {
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
    ) -> Result<Vec<JoinQueueEntry>, sqlx::Error> {
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

    pub async fn dequeue(&self, queued: &[JoinQueueEntry]) -> Result<(), sqlx::Error> {
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
}
