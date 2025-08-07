use axum::{Json, http::StatusCode, response::IntoResponse};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
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

#[derive(Serialize)]
pub struct OtpVerif {
    pub user_id: Uuid,
    pub otp_code: String,
    pub otp_expires_at: DateTime<Utc>,
}

#[derive(Deserialize, Validate)]
pub struct RegisterPayload {
    #[validate(length(min = 3))]
    pub username: String,
    #[validate(email)]
    pub email: String,
}

#[derive(Deserialize, Validate)]
pub struct VerifOtpPayload {
    pub id: String,
    #[validate(length(min = 6, max = 6))]
    pub otp_code: String,
}

pub enum AppResponse {
    Message(String),
    Created(String),
    Redirect(String),
    Updated(String),
}

impl IntoResponse for AppResponse {
    fn into_response(self) -> axum::response::Response {
        let (status, message) = match self {
            AppResponse::Message(m) => (StatusCode::OK, format!("{m}")),
            AppResponse::Created(m) => (StatusCode::CREATED, format!("{m}")),
            AppResponse::Redirect(m) => (StatusCode::OK, format!("{m}")),
            AppResponse::Updated(m) => (StatusCode::OK, format!("{m}")),
        };

        let body = serde_json::json!({"message": message});
        (status, axum::Json(body)).into_response()
    }
}

#[derive(Serialize)]
#[serde(tag = "status")]
pub enum SignUpResponse {
    #[serde(rename = "pending_verification")]
    PendingVerification { message: String, id: Uuid },
}

impl IntoResponse for SignUpResponse {
    fn into_response(self) -> axum::response::Response {
        let status_code = match self {
            _ => StatusCode::OK,
        };
        (status_code, Json(self)).into_response()
    }
}
