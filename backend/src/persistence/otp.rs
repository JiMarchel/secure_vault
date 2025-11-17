use async_trait::async_trait;
use tracing::instrument;
use uuid::Uuid;

use crate::{
    model::{
        app_error::{AppError, AppResult}, otp::OtpRecord,
    },
    persistence::postgres::PostgresPersistence,
    service::otp::OtpPersistence,
};

#[async_trait]
impl OtpPersistence for PostgresPersistence {
    #[instrument(
        name= "persistence.create_otp", 
        skip(self, code, expires_at), 
        fields(user_id=%user_id)
    )]
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
        .map_err(|e| AppError::from(e))?;

        Ok(())
    }

    #[instrument(
        name= "persistence.get_otp_by_user_id", 
        skip(self), 
        fields(user_id=%user_id)
    )]
    async fn get_otp_by_user_id(&self, user_id: Uuid) -> AppResult<OtpRecord> {
        let record = sqlx::query_as::<_, OtpRecord>(
            "SELECT otp_code, otp_expires_at FROM otp_verif WHERE user_id = $1",
        )
        .bind(user_id)
        .fetch_one(&self.pool)
        .await
        .map_err(|e| AppError::from(e))?;

        Ok(record)
    }

    #[instrument(
        name= "persistence.update_otp_by_user_id", 
        skip(self, code, expires_at), 
        fields(user_id=%user_id)
    )]
    async fn update_otp_by_user_id(
        &self,
        user_id: Uuid,
        code: &str,
        expires_at: chrono::DateTime<chrono::Utc>,
    ) -> AppResult<()> {
        sqlx::query("UPDATE otp_verif SET otp_code = $1, otp_expires_at = $2 WHERE user_id = $3")
            .bind(code)
            .bind(expires_at)
            .bind(user_id)
            .execute(&self.pool)
            .await
            .map_err(|e| AppError::from(e))?;

        Ok(())
    }

    #[instrument(
        name= "persistence.delete_otp_by_user_id", 
        skip(self), 
        fields(user_id=%user_id)
    )]
    async fn delete_otp_by_user_id(&self, user_id: Uuid) -> AppResult<()> {
        sqlx::query("DELETE FROM otp_verif WHERE user_id = $1")
            .bind(user_id)
            .execute(&self.pool)
            .await
            .map_err(|e| AppError::from(e))?;
        Ok(())
    }
}
