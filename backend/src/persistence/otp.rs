use async_trait::async_trait;
use uuid::Uuid;

use crate::{
    model::app_error::{AppError, AppResult},
    persistence::postgres::PostgresPersistence,
    service::otp::OtpPersistence,
};

#[async_trait]
impl OtpPersistence for PostgresPersistence {
    async fn create_otp(
        &self,
        user_id: Uuid,
        code: &str,
        expires_at: chrono::DateTime<chrono::Utc>,
    ) -> AppResult<()> {
        sqlx::query(
            "INSERT INTO otp_verif (user_id, otp_code, otp_expires_at) VALUES ($1, $2, $3)",
        )
        .bind(user_id)
        .bind(code)
        .bind(expires_at)
        .execute(&self.pool)
        .await
        .map_err(AppError::from)?;

        Ok(())
    }
}
