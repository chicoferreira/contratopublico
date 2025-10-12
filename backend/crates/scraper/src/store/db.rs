use anyhow::Result;
use common::{Contract, Cpv, Document, Entity};
use sqlx::{PgPool, Postgres, Transaction};

pub async fn insert_contract(contract: &Contract, pg_pool: &PgPool) -> Result<()> {
    let mut tx = pg_pool.begin().await?;

    if let Some(cpv) = &contract.cpv {
        insert_cpv_if_not_exists(&mut tx, cpv).await?;
    }

    let entities = contract
        .contracting
        .iter()
        .chain(contract.contracted.iter())
        .chain(contract.contestants.iter())
        .chain(contract.invitees.iter());

    for entity in entities {
        insert_entity_if_not_exists(&mut tx, entity).await?;
    }

    for document in &contract.documents {
        insert_document_if_not_exists(&mut tx, document).await?;
    }

    insert_contract_main(&mut tx, contract).await?;

    insert_contract_relationships(&mut tx, contract).await?;

    tx.commit().await?;
    Ok(())
}

async fn insert_cpv_if_not_exists(tx: &mut Transaction<'_, Postgres>, cpv: &Cpv) -> Result<()> {
    sqlx::query!(
        "INSERT INTO cpv (code, designation) VALUES ($1, $2) ON CONFLICT (code) DO NOTHING",
        cpv.code,
        cpv.designation
    )
    .execute(&mut **tx)
    .await?;

    Ok(())
}

async fn insert_entity_if_not_exists(
    tx: &mut Transaction<'_, Postgres>,
    entity: &Entity,
) -> Result<()> {
    sqlx::query!(
        "INSERT INTO entities (id, nif) VALUES ($1, $2) ON CONFLICT (id) DO NOTHING",
        entity.id as i64,
        entity.nif
    )
    .execute(&mut **tx)
    .await?;

    Ok(())
}

async fn insert_document_if_not_exists(
    tx: &mut Transaction<'_, Postgres>,
    document: &Document,
) -> Result<()> {
    sqlx::query!(
        "INSERT INTO documents (id, description) VALUES ($1, $2) ON CONFLICT (id) DO NOTHING",
        document.id as i64,
        document.description
    )
    .execute(&mut **tx)
    .await?;

    Ok(())
}

async fn insert_contract_main(
    tx: &mut Transaction<'_, Postgres>,
    contract: &Contract,
) -> Result<()> {
    sqlx::query!(
        r#"
        INSERT INTO contracts (
            id, contracting_procedure_type, publication_date, signing_date,
            ccp, object_brief_description, initial_contractual_price, description,
            cpv_code, regime, contract_status, non_written_contract_justification_types,
            contract_types, execution_deadline_days, execution_place,
            contract_fundamentation_type, contracting_procedure_url, announcement_id,
            direct_award_fundamentation_type, observations, end_of_contract_type,
            close_date, total_effective_price, causes_deadline_change, causes_price_change
        ) VALUES (
            $1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12, $13, $14, $15, $16, $17, $18, $19, $20, $21, $22, $23, $24, $25
        ) ON CONFLICT (id) DO NOTHING
        "#,
        contract.id as i64,
        contract.contracting_procedure_type,
        contract.publication_date,
        contract.signing_date,
        contract.ccp,
        contract.object_brief_description,
        contract.initial_contractual_price.0 as i64,
        contract.description,
        contract.cpv.as_ref().map(|cpv| &cpv.code),
        contract.regime,
        contract.contract_status,
        contract.non_written_contract_justification_types,
        contract.contract_types,
        contract.execution_deadline_days as i32,
        contract.execution_place,
        contract.contract_fundamentation_type,
        contract.contracting_procedure_url,
        contract.announcement_id.map(|id| id as i64),
        contract.direct_award_fundamentation_type,
        contract.observations,
        contract.end_of_contract_type,
        contract.close_date,
        contract.total_effective_price.as_ref().map(|price| price.0 as i64),
        contract.causes_deadline_change,
        contract.causes_price_change
    )
    .execute(&mut **tx)
    .await?;

    Ok(())
}

async fn insert_contract_relationships(
    tx: &mut Transaction<'_, Postgres>,
    contract: &Contract,
) -> Result<()> {
    let contract_id = contract.id as i64;

    for entity in &contract.contracting {
        sqlx::query!(
            "INSERT INTO contract_contracting (contract_id, entity_id, description) VALUES ($1, $2, $3) ON CONFLICT DO NOTHING",
            contract_id,
            entity.id as i64,
            entity.description
        )
        .execute(&mut **tx)
        .await?;
    }

    for entity in &contract.contracted {
        sqlx::query!(
            "INSERT INTO contract_contracted (contract_id, entity_id, description) VALUES ($1, $2, $3) ON CONFLICT DO NOTHING",
            contract_id,
            entity.id as i64,
            entity.description
        )
        .execute(&mut **tx)
        .await?;
    }

    for entity in &contract.contestants {
        sqlx::query!(
            "INSERT INTO contract_contestants (contract_id, entity_id, description) VALUES ($1, $2, $3) ON CONFLICT DO NOTHING",
            contract_id,
            entity.id as i64,
            entity.description
        )
        .execute(&mut **tx)
        .await?;
    }

    for entity in &contract.invitees {
        sqlx::query!(
            "INSERT INTO contract_invitees (contract_id, entity_id, description) VALUES ($1, $2, $3) ON CONFLICT DO NOTHING",
            contract_id,
            entity.id as i64,
            entity.description
        )
        .execute(&mut **tx)
        .await?;
    }

    for document in &contract.documents {
        sqlx::query!(
            "INSERT INTO contract_documents (contract_id, document_id) VALUES ($1, $2) ON CONFLICT DO NOTHING",
            contract_id,
            document.id as i64
        )
        .execute(&mut **tx)
        .await?;
    }

    Ok(())
}
