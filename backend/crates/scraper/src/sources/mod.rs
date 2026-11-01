pub mod base_api;
pub mod base_portal;

use serde::Serialize;
use sqlx::{PgPool, types::Json};

#[derive(Debug, Clone, Copy, sqlx::Type)]
#[sqlx(type_name = "contract_source", rename_all = "snake_case")]
pub enum ContractSource {
    BasePortal,
    BaseApi,
}

pub async fn save_raw_contracts<T: Serialize + Sync>(
    pool: &PgPool,
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
    .execute(pool)
    .await?;

    Ok(())
}
