use axum::{http::StatusCode, response::IntoResponse};
use chrono::{DateTime, Utc};
use serde::{Deserialize};
use sqlx::prelude::FromRow;
use uuid::Uuid;
use validator::Validate;

#[derive(FromRow)]
pub struct User {
    pub id: Uuid,
    pub username: String,
    pub email: String,
    pub encrypted_dek: Option<Vec<u8>>,
    pub salt: Option<Vec<u8>>,
    pub argon2_params: Option<String>,
    pub is_email_verified: bool,
    pub created_at: Option<DateTime<Utc>>,
}


#[derive(Deserialize, Validate)]
pub struct RegisterPayload {
    #[validate(length(min = 3))]
    pub username: String,
    #[validate(email)]
    pub email: String,
}

pub enum AppResponse {
    Message(String),
    Created(String),
    Redirect(String),
}

impl IntoResponse for AppResponse {
    fn into_response(self) -> axum::response::Response {
        let (status, message) = match self {
            AppResponse::Message(m) => (StatusCode::OK, format!("{m}")),
            AppResponse::Created(m) => (StatusCode::CREATED, format!("{m}")),
            AppResponse::Redirect(m) => (StatusCode::OK, format!("{m}")),
        };

        let body = serde_json::json!({"message": message});
        (status, axum::Json(body)).into_response()
    }
}
