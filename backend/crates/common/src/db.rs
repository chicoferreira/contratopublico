use anyhow::Context;
use chrono::NaiveDate;
use sqlx::{PgConnection, PgPool};

use crate::{Contract, ContractType, Cpv, Currency, Document, Entity, ProcedureType};

struct ContractMainRow {
    id: i64,
    procedure_type: Option<ProcedureType>,
    publication_date: NaiveDate,
    signing_date: Option<NaiveDate>,
    object_brief_description: String,
    initial_contractual_price: i64,
    description: Option<String>,
    regime: Option<String>,
    contract_status: Option<String>,
    non_written_contract_justification_types: Vec<String>,
    contract_types: Vec<ContractType>,
    execution_deadline_days: i32,
    execution_places: Vec<String>,
    contract_fundamentation_type: String,
    contracting_procedure_url: Option<String>,
    announcement_id: Option<i64>,
    direct_award_fundamentation_type: Option<String>,
    observations: Option<String>,
    end_of_contract_type: Option<String>,
    close_date: Option<NaiveDate>,
    total_effective_price: Option<i64>,
    causes_deadline_change: Option<String>,
    causes_price_change: Option<String>,
    estimated_base_price: Option<i64>,
    award_decision_date: Option<NaiveDate>,
    framework_agreement_description: Option<String>,
    special_measures: Option<String>,
    ambient_criteria: bool,
}

struct EntityRow {
    id: i64,
    nif: String,
    description: String,
}

struct DocumentRow {
    id: i64,
    description: String,
}

impl From<EntityRow> for Entity {
    fn from(row: EntityRow) -> Self {
        Entity {
            id: row.id as u64,
            nif: row.nif,
            description: row.description,
        }
    }
}

impl From<DocumentRow> for Document {
    fn from(row: DocumentRow) -> Self {
        Document {
            id: row.id as u64,
            description: row.description,
        }
    }
}

#[derive(Debug, Clone)]
pub struct ContractDatabase {
    pub(crate) pool: PgPool,
}

#[derive(clap::Parser)]
pub struct PostgresConfig {
    #[clap(long, env = "POSTGRES_HOST", default_value = "localhost")]
    pub postgres_host: String,
    #[clap(long, env = "POSTGRES_PORT", default_value = "5432")]
    pub postgres_port: u16,
    #[clap(long, env = "POSTGRES_USER", default_value = "contratopublico")]
    pub postgres_user: String,
    #[clap(long, env = "POSTGRES_PASSWORD", default_value = "contratopublico")]
    pub postgres_password: String,
    #[clap(long, env = "POSTGRES_DB", default_value = "contratopublico")]
    pub postgres_db: String,
}

impl PostgresConfig {
    pub async fn connect(&self) -> anyhow::Result<PgPool> {
        let options = sqlx::postgres::PgConnectOptions::new()
            .host(&self.postgres_host)
            .port(self.postgres_port)
            .username(&self.postgres_user)
            .password(&self.postgres_password)
            .database(&self.postgres_db);

        let pg_pool = PgPool::connect_with(options)
            .await
            .context("Failed to connect to database")?;

        sqlx::migrate!("../../migrations")
            .run(&pg_pool)
            .await
            .context("Failed to run migrations")?;

        Ok(pg_pool)
    }
}

impl ContractDatabase {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }

    pub async fn get_contract(&self, id: u64) -> Result<Option<Contract>, sqlx::Error> {
        let main: Option<ContractMainRow> = sqlx::query_as!(
            ContractMainRow,
            r#"
            SELECT
                id, procedure_type AS "procedure_type: ProcedureType", publication_date,
                signing_date, object_brief_description, initial_contractual_price, description,
                regime, contract_status, non_written_contract_justification_types,
                contract_types AS "contract_types: Vec<ContractType>", execution_deadline_days,
                execution_places, contract_fundamentation_type, contracting_procedure_url,
                announcement_id, direct_award_fundamentation_type, observations,
                end_of_contract_type, close_date, total_effective_price, causes_deadline_change,
                causes_price_change, estimated_base_price, award_decision_date,
                framework_agreement_description, special_measures, ambient_criteria
            FROM contracts
            WHERE id = $1
            "#,
            id as i64
        )
        .fetch_optional(&self.pool)
        .await?;

        let Some(main) = main else { return Ok(None) };

        let contracting_fut = sqlx::query_as!(
            EntityRow,
            r#"
            SELECT e.id, e.nif, cc.description
            FROM contract_contracting cc
            JOIN entities e ON e.id = cc.entity_id
            WHERE cc.contract_id = $1
            "#,
            main.id
        )
        .fetch_all(&self.pool);

        let contracted_fut = sqlx::query_as!(
            EntityRow,
            r#"
            SELECT e.id, e.nif, cc.description
            FROM contract_contracted cc
            JOIN entities e ON e.id = cc.entity_id
            WHERE cc.contract_id = $1
            "#,
            main.id
        )
        .fetch_all(&self.pool);

        let contestants_fut = sqlx::query_as!(
            EntityRow,
            r#"
            SELECT e.id, e.nif, cc.description
            FROM contract_contestants cc
            JOIN entities e ON e.id = cc.entity_id
            WHERE cc.contract_id = $1
            "#,
            main.id
        )
        .fetch_all(&self.pool);

        let invitees_fut = sqlx::query_as!(
            EntityRow,
            r#"
            SELECT e.id, e.nif, cc.description
            FROM contract_invitees cc
            JOIN entities e ON e.id = cc.entity_id
            WHERE cc.contract_id = $1
            "#,
            main.id
        )
        .fetch_all(&self.pool);

        let documents_fut = sqlx::query_as!(
            DocumentRow,
            r#"
            SELECT d.id, d.description
            FROM contract_documents cd
            JOIN documents d ON d.id = cd.document_id
            WHERE cd.contract_id = $1
            "#,
            main.id
        )
        .fetch_all(&self.pool);

        let cpvs_fut = sqlx::query_as!(
            Cpv,
            r#"
            SELECT c.code, c.designation
            FROM contract_cpvs cc
            JOIN cpv c ON c.code = cc.cpv_code
            WHERE cc.contract_id = $1
            "#,
            main.id
        )
        .fetch_all(&self.pool);

        let (
            contracting_rows,
            contracted_rows,
            contestants_rows,
            invitees_rows,
            documents_rows,
            cpvs,
        ) = tokio::try_join!(
            contracting_fut,
            contracted_fut,
            contestants_fut,
            invitees_fut,
            documents_fut,
            cpvs_fut
        )?;

        let contracting: Vec<Entity> = contracting_rows.into_iter().map(Into::into).collect();
        let contracted: Vec<Entity> = contracted_rows.into_iter().map(Into::into).collect();
        let contestants: Vec<Entity> = contestants_rows.into_iter().map(Into::into).collect();
        let invitees: Vec<Entity> = invitees_rows.into_iter().map(Into::into).collect();
        let documents: Vec<Document> = documents_rows.into_iter().map(Into::into).collect();

        Ok(Some(Contract {
            id: main.id as u64,
            procedure_type: main.procedure_type,
            publication_date: main.publication_date,
            signing_date: main.signing_date,
            object_brief_description: main.object_brief_description,
            initial_contractual_price: Currency(main.initial_contractual_price as isize),
            description: main.description,
            contracting,
            contracted,
            cpvs,
            regime: main.regime,
            contract_status: main.contract_status,
            non_written_contract_justification_types: main.non_written_contract_justification_types,
            contract_types: main.contract_types,
            execution_deadline_days: main.execution_deadline_days as usize,
            execution_places: main.execution_places,
            contract_fundamentation_type: main.contract_fundamentation_type,
            contestants,
            invitees,
            documents,
            contracting_procedure_url: main.contracting_procedure_url,
            announcement_id: main.announcement_id.map(|v| v as usize),
            direct_award_fundamentation_type: main.direct_award_fundamentation_type,
            observations: main.observations,
            end_of_contract_type: main.end_of_contract_type,
            close_date: main.close_date,
            total_effective_price: main.total_effective_price.map(|v| Currency(v as isize)),
            causes_deadline_change: main.causes_deadline_change,
            causes_price_change: main.causes_price_change,
            estimated_base_price: main.estimated_base_price.map(|v| Currency(v as isize)),
            award_decision_date: main.award_decision_date,
            framework_agreement_description: main.framework_agreement_description,
            special_measures: main.special_measures,
            ambient_criteria: main.ambient_criteria,
        }))
    }

    pub async fn delete_contracts(&self, ids: &[i64]) -> Result<(), sqlx::Error> {
        // ON DELETE CASCADE also deletes every row of these contracts in the relationship tables
        sqlx::query("DELETE FROM contracts WHERE id = ANY($1)")
            .bind(ids)
            .execute(&self.pool)
            .await?;

        Ok(())
    }

    /// Deletes every contract, along with the entities, CPVs and documents.
    pub async fn delete_all_contracts(&self) -> Result<(), sqlx::Error> {
        sqlx::query("TRUNCATE contracts, entities, cpv, documents CASCADE")
            .execute(&self.pool)
            .await?;

        Ok(())
    }

    pub async fn insert_contracts(&self, contracts: &[Contract]) -> Result<(), sqlx::Error> {
        let mut tx = self.pool.begin().await?;

        for contract in contracts {
            insert_contract(&mut tx, contract).await?;
        }

        tx.commit().await
    }
}

async fn insert_contract(tx: &mut PgConnection, contract: &Contract) -> Result<(), sqlx::Error> {
    // ON DELETE CASCADE also deletes every row of this contract in the relationship tables
    sqlx::query("DELETE FROM contracts WHERE id = $1")
        .bind(contract.id as i64)
        .execute(&mut *tx)
        .await?;

    for cpv in &contract.cpvs {
        sqlx::query!(
            "INSERT INTO cpv (code, designation) VALUES ($1, $2) ON CONFLICT (code) DO NOTHING",
            cpv.code,
            cpv.designation
        )
        .execute(&mut *tx)
        .await?;
    }

    for entity in contract
        .contracting
        .iter()
        .chain(contract.contracted.iter())
        .chain(contract.contestants.iter())
        .chain(contract.invitees.iter())
    {
        sqlx::query!(
            "INSERT INTO entities (id, nif) VALUES ($1, $2) ON CONFLICT (id) DO NOTHING",
            entity.id as i64,
            entity.nif
        )
        .execute(&mut *tx)
        .await?;
    }

    for document in &contract.documents {
        sqlx::query!(
            "INSERT INTO documents (id, description) VALUES ($1, $2) ON CONFLICT (id) DO NOTHING",
            document.id as i64,
            document.description
        )
        .execute(&mut *tx)
        .await?;
    }

    sqlx::query!(
            r#"
            INSERT INTO contracts (
                id, procedure_type, publication_date, signing_date,
                object_brief_description, initial_contractual_price, description, regime,
                contract_status, non_written_contract_justification_types, contract_types,
                execution_deadline_days, execution_places, contract_fundamentation_type,
                contracting_procedure_url, announcement_id, direct_award_fundamentation_type,
                observations, end_of_contract_type, close_date, total_effective_price,
                causes_deadline_change, causes_price_change, estimated_base_price,
                award_decision_date, framework_agreement_description, special_measures,
                ambient_criteria
            ) VALUES (
                $1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12, $13, $14, $15, $16, $17, $18, $19, $20, $21, $22, $23,
                $24, $25, $26, $27, $28
            ) ON CONFLICT (id) DO NOTHING
            "#,
            contract.id as i64,
            contract.procedure_type as Option<ProcedureType>,
            contract.publication_date,
            contract.signing_date,
            contract.object_brief_description,
            contract.initial_contractual_price.0 as i64,
            contract.description,
            contract.regime,
            contract.contract_status,
            &contract.non_written_contract_justification_types,
            &contract.contract_types as &[ContractType],
            contract.execution_deadline_days as i32,
            &contract.execution_places,
            contract.contract_fundamentation_type,
            contract.contracting_procedure_url,
            contract.announcement_id.map(|id| id as i64),
            contract.direct_award_fundamentation_type,
            contract.observations,
            contract.end_of_contract_type,
            contract.close_date,
            contract.total_effective_price.as_ref().map(|price| price.0 as i64),
            contract.causes_deadline_change,
            contract.causes_price_change,
            contract.estimated_base_price.as_ref().map(|price| price.0 as i64),
            contract.award_decision_date,
            contract.framework_agreement_description,
            contract.special_measures,
            contract.ambient_criteria
        )
        .execute(&mut *tx)
        .await?;

    let contract_id = contract.id as i64;

    for entity in &contract.contracting {
        sqlx::query!(
                "INSERT INTO contract_contracting (contract_id, entity_id, description) VALUES ($1, $2, $3) ON CONFLICT DO NOTHING",
                contract_id,
                entity.id as i64,
                entity.description
            )
            .execute(&mut *tx)
            .await?;
    }

    for entity in &contract.contracted {
        sqlx::query!(
                "INSERT INTO contract_contracted (contract_id, entity_id, description) VALUES ($1, $2, $3) ON CONFLICT DO NOTHING",
                contract_id,
                entity.id as i64,
                entity.description
            )
            .execute(&mut *tx)
            .await?;
    }

    for entity in &contract.contestants {
        sqlx::query!(
                "INSERT INTO contract_contestants (contract_id, entity_id, description) VALUES ($1, $2, $3) ON CONFLICT DO NOTHING",
                contract_id,
                entity.id as i64,
                entity.description
            )
            .execute(&mut *tx)
            .await?;
    }

    for entity in &contract.invitees {
        sqlx::query!(
                "INSERT INTO contract_invitees (contract_id, entity_id, description) VALUES ($1, $2, $3) ON CONFLICT DO NOTHING",
                contract_id,
                entity.id as i64,
                entity.description
            )
            .execute(&mut *tx)
            .await?;
    }

    for document in &contract.documents {
        sqlx::query!(
                "INSERT INTO contract_documents (contract_id, document_id) VALUES ($1, $2) ON CONFLICT DO NOTHING",
                contract_id,
                document.id as i64
            )
            .execute(&mut *tx)
            .await?;
    }

    for cpv in &contract.cpvs {
        sqlx::query!(
                "INSERT INTO contract_cpvs (contract_id, cpv_code) VALUES ($1, $2) ON CONFLICT DO NOTHING",
                contract_id,
                cpv.code
            )
            .execute(&mut *tx)
            .await?;
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use sqlx::PgPool;

    use crate::{
        Contract, ContractType, Cpv, Currency, Document, Entity, ProcedureType,
        db::ContractDatabase,
    };

    #[sqlx::test(migrations = "../../migrations")]
    async fn test_db(pg_pool: PgPool) -> sqlx::Result<()> {
        let contract = Contract {
            id: 1,
            procedure_type: Some(ProcedureType::DirectAward),
            publication_date: chrono::NaiveDate::from_ymd_opt(2023, 1, 15).unwrap(),
            signing_date: Some(chrono::NaiveDate::from_ymd_opt(2023, 2, 1).unwrap()),
            object_brief_description: "Test contract for software development".to_string(),
            initial_contractual_price: Currency(100000),
            description: Some("Detailed description of the contract".to_string()),
            contracting: vec![Entity {
                id: 1,
                nif: "123456789".to_string(),
                description: "Test Contracting Entity".to_string(),
            }],
            contracted: vec![Entity {
                id: 2,
                nif: "987654321".to_string(),
                description: "Test Contracted Entity".to_string(),
            }],
            cpvs: vec![Cpv {
                code: "72000000".to_string(),
                designation: "IT services".to_string(),
            }],
            regime: Some("Public".to_string()),
            contract_status: Some("Active".to_string()),
            non_written_contract_justification_types: vec!["Emergency".to_string()],
            contract_types: vec![ContractType::GoodsAcquisition, ContractType::Other],
            execution_deadline_days: 90,
            execution_places: vec!["Lisbon, Portugal".to_string()],
            contract_fundamentation_type: "Legal basis".to_string(),
            contestants: vec![],
            invitees: vec![],
            documents: vec![Document {
                id: 1,
                description: "Contract specification document".to_string(),
            }],
            contracting_procedure_url: Some("https://example.com/procedure".to_string()),
            announcement_id: Some(12345),
            direct_award_fundamentation_type: Some("Urgency".to_string()),
            observations: Some("Additional observations".to_string()),
            end_of_contract_type: Some("Completion".to_string()),
            close_date: Some(chrono::NaiveDate::from_ymd_opt(2023, 5, 1).unwrap()),
            total_effective_price: Some(Currency(95000)),
            causes_deadline_change: Some("Technical delays".to_string()),
            causes_price_change: Some("Scope reduction".to_string()),
            estimated_base_price: Some(Currency(120000)),
            award_decision_date: Some(chrono::NaiveDate::from_ymd_opt(2023, 1, 20).unwrap()),
            framework_agreement_description: Some("Framework agreement".to_string()),
            special_measures: Some("PRR".to_string()),
            ambient_criteria: true,
        };

        let db = ContractDatabase::new(pg_pool);

        assert_eq!(None, db.get_contract(contract.id).await?);

        db.insert_contracts(std::slice::from_ref(&contract)).await?;

        let contract_from_db = db.get_contract(contract.id).await?;
        assert_eq!(Some(contract.clone()), contract_from_db);

        db.delete_all_contracts().await?;

        assert_eq!(None, db.get_contract(contract.id).await?);
        let entities: i64 = sqlx::query_scalar("SELECT count(*) FROM entities")
            .fetch_one(&db.pool)
            .await?;
        assert_eq!(entities, 0);

        Ok(())
    }
}
