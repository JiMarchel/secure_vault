use async_trait::async_trait;
use uuid::Uuid;

use crate::{
    model::{app_error::{AppError, AppResult}, user::OtpRecord},
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

    async fn get_otp_by_user_id(&self, user_id: Uuid) -> AppResult<OtpRecord> {
        let record = sqlx::query_as::<_, OtpRecord>(
            "SELECT otp_code, otp_expires_at FROM otp_verif WHERE user_id = $1",
        )
        .bind(user_id)
        .fetch_one(&self.pool)
        .await
        .map_err(AppError::from)?;

        Ok(record)
    }

    async fn update_otp_by_user_id(
        &self,
        user_id: Uuid,
        code: &str,
        expires_at: chrono::DateTime<chrono::Utc>,
    ) -> AppResult<()> {
        sqlx::query(
            "UPDATE otp_verif SET otp_code = $1, otp_expires_at = $2 WHERE user_id = $3",
        )
        .bind(code)
        .bind(expires_at)
        .bind(user_id)
        .execute(&self.pool)
        .await
        .map_err(AppError::from)?;

        Ok(())
    }   
}
