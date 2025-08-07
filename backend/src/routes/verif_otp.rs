use std::sync::Arc;

use axum::{Json, extract::State};
use uuid::Uuid;
use validator::Validate;

use crate::{
    error::AppError,
    models::{SignUpResponse, VerifOtpPayload},
    startup::ApplicationState,
};

pub async fn verif_otp(
    State(app_state): State<Arc<ApplicationState>>,
    Json(payload): Json<VerifOtpPayload>,
) -> Result<Json<SignUpResponse>, AppError> {
    payload.validate()?;

    let id = Uuid::try_parse(&payload.id)
        .map_err(|_| AppError::BadRequest("Not valid Uuid format".to_string()))?;

    let result = sqlx::query!(
        r#"
        SELECT ov.user_id
        FROM otp_verif ov
        JOIN users u ON ov.user_id = u.id
        WHERE u.id = $1 AND ov.otp_code = $2 AND ov.otp_expires_at > NOW()
        "#,
        id,
        payload.otp_code
    )
    .fetch_optional(&app_state.pool)
    .await?;

    let user_id = match result {
        Some(row) => row.user_id,
        None => {
            return Err(AppError::BadRequest(
                "Wrong OTP code or it's already expires.".to_string(),
            ));
        }
    };

    let mut tx = app_state.pool.begin().await?;

    sqlx::query!(
        "UPDATE users SET is_email_verified = true WHERE id = $1",
        user_id
    )
    .execute(&mut *tx)
    .await?;

    sqlx::query!("DELETE FROM otp_verif WHERE user_id = $1", user_id)
        .execute(&mut *tx)
        .await?;

    tx.commit().await?;

    Ok(Json(SignUpResponse::PendingVerification {
        message: "verif_password".to_string(),
        id: user_id,
    }))
}
