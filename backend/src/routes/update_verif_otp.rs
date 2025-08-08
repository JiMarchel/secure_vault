use std::sync::Arc;

use axum::extract::{Path, State};
use chrono::{Duration, Utc};
use rand::{Rng, rng};
use uuid::Uuid;

use crate::{error::AppError, models::AppResponse, startup::ApplicationState};

pub async fn update_verif_otp(
    State(app_state): State<Arc<ApplicationState>>,
    Path(id): Path<String>,
) -> Result<AppResponse, AppError> {
    //SEND MAIL
    let user_id = Uuid::try_parse(&id)
        .map_err(|_| AppError::BadRequest("Invalid Uuid format".to_string()))?;

    let otp_code = format!("{:06}", rng().random_range(1..1_000_000));
    let expires_at = Utc::now() + Duration::minutes(10);

    sqlx::query!(
        "UPDATE otp_verif SET otp_code = $1, otp_expires_at = $2 WHERE user_id = $3",
        otp_code,
        expires_at,
        user_id
    )
    .execute(&app_state.pool)
    .await?;

    Ok(AppResponse::Updated("OTP Updated".into()))
}
