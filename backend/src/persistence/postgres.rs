use sqlx::PgPool;

use crate::model::app_error::AppError;

pub struct PostgresPersistence {
    pub pool: PgPool,
}

impl PostgresPersistence {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}

impl From<sqlx::Error> for  AppError{
    fn from(value: sqlx::Error) -> Self {
        AppError::Database(value.to_string())
    }
}