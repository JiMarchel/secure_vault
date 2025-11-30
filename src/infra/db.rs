use sqlx::{PgPool, postgres::PgPoolOptions};

use crate::{infra::config::DatabaseConfig, persistence::postgres::PostgresPersistence};

pub async fn get_connection_pool(configuration: &DatabaseConfig) -> anyhow::Result<PgPool> {
    Ok(PgPoolOptions::new()
        .max_connections(10)
        .connect_with(configuration.with_db())
        .await?)
}

pub async fn postgres_persistance(configuration: &DatabaseConfig) -> anyhow::Result<PostgresPersistence> {
    let pool = get_connection_pool(configuration).await?;
    Ok(PostgresPersistence::new(pool))
}
