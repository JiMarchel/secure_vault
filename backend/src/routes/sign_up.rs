use axum::{Json, extract::State};
use chrono::{Duration, Utc};
use rand::{Rng, rng};
use std::sync::Arc;
use uuid::Uuid;
use validator::Validate;

use crate::{
    error::AppError,
    models::{GenericResponse, RegisterPayload},
    startup::ApplicationState,
};

pub async fn sign_up(
    State(app_state): State<Arc<ApplicationState>>,
    Json(payload): Json<RegisterPayload>,
) -> Result<Json<GenericResponse>, AppError> {
    payload.validate()?;

    let existing_user: Option<Uuid> = sqlx::query_scalar("SELECT id FROM users WHERE email = $1")
        .bind(&payload.email)
        .fetch_optional(&app_state.pool)
        .await?;

    if existing_user.is_some() {
        return Err(AppError::BadRequest("Email already taken".to_string()));
    }

    let user_id: Uuid =
        sqlx::query_scalar("INSERT INTO users (username, email) VALUES($1, $2) RETURNING id")
            .bind(&payload.username)
            .bind(&payload.email)
            .fetch_one(&app_state.pool)
            .await?;

    let otp_code = format!("{:06}", rng().random_range(1..1_000_000));
    let expires_at = Utc::now() + Duration::minutes(5);

    sqlx::query("INSERT INTO otp_verif (user_id, otp_code, otp_expires_at) VALUES ($1, $2, $3)")
        .bind(user_id)
        .bind(&otp_code)
        .bind(expires_at)
        .execute(&app_state.pool)
        .await?;

    Ok(Json(GenericResponse {
        message: "Success created account!".to_string(),
    }))
}
