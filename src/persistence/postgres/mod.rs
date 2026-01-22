pub mod jwt;
pub mod user;
pub mod vault;

use sqlx::PgPool;

pub struct PostgresPersistence {
    pub pool: PgPool,
}

impl PostgresPersistence {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}
