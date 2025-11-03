use std::env;

use anyhow::Ok;
use sqlx::{PgPool, postgres::PgPoolOptions};

use crate::persistence::postgres::PostgresPersistence;


pub async fn init_db() -> anyhow::Result<PgPool> {
    let db_url = env::var("DATABASE_URL")?;
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&db_url)
        .await?;

    Ok(pool)
}

pub async fn postgres_persistance() -> anyhow::Result<PostgresPersistence> {
    let pool = init_db().await?;
    let persistance = PostgresPersistence::new(pool);
    Ok(persistance)
}
