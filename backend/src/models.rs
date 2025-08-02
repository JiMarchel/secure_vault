use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::prelude::FromRow;
use uuid::Uuid;
use validator::Validate;

#[derive(Debug, FromRow)]
pub struct User {
    pub id: Uuid,
    pub username: Option<String>,
    pub email: String,
    pub encrypted_dek: Option<Vec<u8>>,
    pub salt: Option<Vec<u8>>,
    pub argon2_params: Option<String>,
    pub is_email_verified: bool,
    pub created_at: Option<DateTime<Utc>>,
}

#[derive(Deserialize, Debug, Validate)]
pub struct RegisterPayload {
    #[validate(length(min = 3))]
    pub username: String,
    #[validate(email)]
    pub email: String,
}

#[derive(Serialize)]
pub struct GenericResponse {
    pub message: String,
}
