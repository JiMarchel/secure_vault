use std::sync::Arc;

use axum::{
    Json,
    extract::{Path, State},
};
use uuid::Uuid;

use crate::{error::AppError, models::OtpVerif, startup::ApplicationState};

pub async fn get_verif_otp(
    State(app_state): State<Arc<ApplicationState>>,
    Path(id): Path<String>,
) -> Result<Json<OtpVerif>, AppError> {
    let user_id = Uuid::try_parse(&id)
        .map_err(|_| AppError::BadRequest("Invalid Uuid format".to_string()))?;

    let otp_data = sqlx::query_as!(
        OtpVerif,
        "SELECT user_id, otp_code, otp_expires_at FROM otp_verif WHERE user_id = $1",
        user_id
    )
    .fetch_optional(&app_state.pool)
    .await?;

    if let Some(data) = otp_data {
        Ok(Json(data))
    } else {
        Err(AppError::NotFound)
    }
}
