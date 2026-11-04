use serde::{Serialize, de::DeserializeOwned};
use sqlx::{PgPool, Postgres, Transaction, postgres::PgListener, types::Json};

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

/// A queued contract with its data from each source, which is `None` if that source doesn't have it.
#[derive(Debug, sqlx::FromRow)]
pub struct QueuedContract<Api, Portal> {
    #[sqlx(flatten)]
    pub entry: QueueEntry,
    #[sqlx(json(nullable))]
    pub api: Option<Api>,
    #[sqlx(json(nullable))]
    pub portal: Option<Portal>,
}

/// A full redownload of a source, used to make the saved contracts of that source match it
/// exactly, including deleting the contracts that the source no longer has.
///
/// To accomplish this, the contracts are added in batches to a temporary `snapshot` table.
/// [`Snapshot::merge`] then compares it with `raw_contracts` in a single `MERGE`, so every
/// contract of the source that is not in the `snapshot` table is also deleted from
/// `raw_contracts`.
///
/// Partial downloads must use [`RawContractDatabase::save_contracts`] instead, which never
/// deletes contracts.
///
/// Everything runs in one transaction, so nothing is saved until [`Snapshot::commit`].
pub struct Snapshot {
    tx: Transaction<'static, Postgres>,
    source: ContractSource,
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
        let (ids, data) = unzip_contracts(contracts.iter().map(|(id, contract)| (*id, contract)));

        // Only update the row if the data changed, so the triggers don't queue unchanged contracts.
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

    pub async fn begin_snapshot(&self, source: ContractSource) -> Result<Snapshot, sqlx::Error> {
        let mut tx = self.pool.begin().await?;

        sqlx::query(
            r#"
            CREATE TEMP TABLE snapshot (
                contract_id BIGINT PRIMARY KEY,
                data JSONB NOT NULL
            ) ON COMMIT DROP
            "#,
        )
        .execute(&mut *tx)
        .await?;

        Ok(Snapshot { tx, source })
    }

    pub async fn delete_contract(
        &self,
        source: ContractSource,
        id: u64,
    ) -> Result<(), sqlx::Error> {
        sqlx::query("DELETE FROM raw_contracts WHERE source = $1 AND contract_id = $2")
            .bind(source)
            .bind(id as i64)
            .execute(&self.pool)
            .await?;

        Ok(())
    }

    /// Returns the queued contracts before `before_id`, newest first, with their data from each source.
    pub async fn list_queued_before<Api, Portal>(
        &self,
        before_id: i64,
        limit: i64,
    ) -> Result<Vec<QueuedContract<Api, Portal>>, sqlx::Error>
    where
        Api: DeserializeOwned + Send + Unpin + 'static,
        Portal: DeserializeOwned + Send + Unpin + 'static,
    {
        sqlx::query_as(
            r#"
            SELECT q.contract_id, q.version, api.data AS api, portal.data AS portal
            FROM join_queue q
            LEFT JOIN raw_contracts api
                ON api.source = $1 AND api.contract_id = q.contract_id
            LEFT JOIN raw_contracts portal
                ON portal.source = $2 AND portal.contract_id = q.contract_id
            WHERE q.contract_id < $3
            ORDER BY q.contract_id DESC
            LIMIT $4
            "#,
        )
        .bind(ContractSource::BaseApi)
        .bind(ContractSource::BasePortal)
        .bind(before_id)
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

    /// Returns a listener that is notified every time contracts are added to the join queue.
    pub async fn listen_join_queue(&self) -> Result<PgListener, sqlx::Error> {
        let mut listener = PgListener::connect_with(&self.pool).await?;
        listener.listen("join_queue").await?;

        Ok(listener)
    }

    /// Returns the queued contract with the highest id (the newest), skipping `excluded_ids`.
    pub async fn next_portal_refetch(
        &self,
        excluded_ids: &[i64],
    ) -> Result<Option<QueueEntry>, sqlx::Error> {
        sqlx::query_as(
            r#"
            SELECT contract_id, version
            FROM portal_refetch_queue
            WHERE contract_id <> ALL($1)
            ORDER BY contract_id DESC
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

impl Snapshot {
    /// Adds a batch of contracts to the snapshot, without touching `raw_contracts` yet.
    pub async fn add<'a, T: Serialize + Sync + 'a>(
        &mut self,
        contracts: impl IntoIterator<Item = (u64, &'a T)>,
    ) -> Result<(), sqlx::Error> {
        let (ids, data) = unzip_contracts(contracts);

        sqlx::query(
            r#"
            INSERT INTO snapshot
            SELECT * FROM UNNEST($1::BIGINT[], $2::JSONB[])
            ON CONFLICT (contract_id) DO NOTHING
            "#,
        )
        .bind(ids)
        .bind(data)
        .execute(&mut *self.tx)
        .await?;

        Ok(())
    }

    /// Adds, updates and deletes the saved contracts of the source so they match the snapshot,
    /// and returns how many were deleted. Nothing is saved until [`Snapshot::commit`].
    ///
    /// Only rows whose data changed are updated, so the triggers don't queue unchanged contracts.
    /// Rows of other sources are never deleted.
    /// Must be called once, after every contract was added.
    pub async fn merge(&mut self) -> Result<i64, sqlx::Error> {
        sqlx::query_scalar(
            r#"
            WITH merged AS (
                MERGE INTO raw_contracts r
                USING snapshot s ON r.source = $1 AND r.contract_id = s.contract_id
                WHEN MATCHED AND r.data IS DISTINCT FROM s.data THEN
                    UPDATE SET data = s.data, fetched_at = now()
                WHEN NOT MATCHED THEN
                    INSERT (source, contract_id, data) VALUES ($1, s.contract_id, s.data)
                WHEN NOT MATCHED BY SOURCE AND r.source = $1 THEN
                    DELETE
                RETURNING merge_action() AS action
            )
            SELECT count(*) FILTER (WHERE action = 'DELETE') FROM merged
            "#,
        )
        .bind(self.source)
        .fetch_one(&mut *self.tx)
        .await
    }

    /// Saves the merge and drops the temporary table.
    pub async fn commit(self) -> Result<(), sqlx::Error> {
        self.tx.commit().await
    }
}

fn unzip_contracts<'a, T>(
    contracts: impl IntoIterator<Item = (u64, &'a T)>,
) -> (Vec<i64>, Vec<Json<&'a T>>) {
    contracts
        .into_iter()
        .map(|(id, contract)| (id as i64, Json(contract)))
        .unzip()
}

#[cfg(test)]
mod tests {
    use std::time::Duration;

    use serde_json::{Value, json};
    use sqlx::PgPool;

    use super::RawContractDatabase;
    use crate::sources::ContractSource;

    async fn contract_ids(pool: &PgPool, table: &str) -> Vec<i64> {
        sqlx::query_scalar(&format!(
            "SELECT contract_id FROM {table} ORDER BY contract_id"
        ))
        .fetch_all(pool)
        .await
        .unwrap()
    }

    #[sqlx::test(migrations = "../../migrations")]
    async fn snapshot_merge_only_changes_its_source(pool: PgPool) -> sqlx::Result<()> {
        let db = RawContractDatabase::new(pool.clone());

        db.save_contracts(
            ContractSource::BaseApi,
            &[
                (1, json!({"v": 1})),
                (2, json!({"v": 2})),
                (3, json!({"v": 3})),
            ],
        )
        .await?;
        db.save_contracts(ContractSource::BasePortal, &[(1, json!({"p": 1}))])
            .await?;
        sqlx::query("DELETE FROM join_queue").execute(&pool).await?;
        let mut listener = db.listen_portal_refetches().await?;

        let mut snapshot = db.begin_snapshot(ContractSource::BaseApi).await?;
        let changed = json!({"v": 20});
        let unchanged = json!({"v": 3});
        let added = json!({"v": 4});
        snapshot
            .add([(2, &changed), (3, &unchanged), (4, &added)])
            .await?;
        assert_eq!(snapshot.merge().await?, 1);
        snapshot.commit().await?;

        let rows: Vec<(String, i64, Value)> = sqlx::query_as(
            "SELECT source::TEXT, contract_id, data FROM raw_contracts ORDER BY source, contract_id",
        )
        .fetch_all(&pool)
        .await?;
        assert_eq!(
            rows,
            [
                ("base_api".to_string(), 2, changed),
                ("base_api".to_string(), 3, unchanged),
                ("base_api".to_string(), 4, added),
                ("base_portal".to_string(), 1, json!({"p": 1})),
            ]
        );

        assert_eq!(contract_ids(&pool, "join_queue").await, [1, 2, 4]);
        assert_eq!(contract_ids(&pool, "portal_refetch_queue").await, [1, 2]);
        tokio::time::timeout(Duration::from_secs(5), listener.recv())
            .await
            .expect("Queueing didn't notify the portal scraper")?;

        Ok(())
    }

    #[sqlx::test(migrations = "../../migrations")]
    async fn list_queued_before_pairs_both_sources(pool: PgPool) -> sqlx::Result<()> {
        let db = RawContractDatabase::new(pool);
        let (api, portal) = (ContractSource::BaseApi, ContractSource::BasePortal);

        db.save_contracts(api, &[(1, 10), (2, 20), (4, 40)]).await?;
        db.save_contracts(portal, &[(2, 200), (3, 300)]).await?;
        db.delete_contract(api, 1).await?;

        let queued = db.list_queued_before::<i64, i64>(4, 10).await?;
        let queued: Vec<_> = queued
            .iter()
            .map(|c| (c.entry.contract_id, c.api, c.portal))
            .collect();

        assert_eq!(
            queued,
            [
                (3, None, Some(300)),
                (2, Some(20), Some(200)),
                (1, None, None)
            ]
        );

        Ok(())
    }
}
