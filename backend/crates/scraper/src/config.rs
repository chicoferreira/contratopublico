use anyhow::Context;
use sqlx::PgPool;

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
    pub async fn create_pool(&self) -> anyhow::Result<PgPool> {
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

#[derive(clap::Parser)]
pub struct MeilisearchConfig {
    #[clap(long, env, default_value = "http://localhost:7700")]
    pub meilisearch_url: String,
    #[clap(long, env = "MEILI_MASTER_KEY", default_value = "masterKey")]
    pub meilisearch_api_key: Option<String>,
}

impl MeilisearchConfig {
    pub fn create_client(&self) -> anyhow::Result<meilisearch_sdk::client::Client> {
        meilisearch_sdk::client::Client::new(
            self.meilisearch_url.clone(),
            self.meilisearch_api_key.clone(),
        )
        .context("Failed to create Meilisearch client")
    }
}
