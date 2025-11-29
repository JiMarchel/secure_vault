use async_trait::async_trait;
use tracing::instrument;
use uuid::Uuid;

use crate::{
    model::app_error::AppResult, persistence::postgres::PostgresPersistence,
    service::jwt::JwtPersistence,
};

#[async_trait]
impl JwtPersistence for PostgresPersistence {
    #[instrument(
        name = "persistence.create_refresh_token",
        skip(self, refresh_token, user_id)
    )]
    async fn create_refresh_token(&self, user_id: Uuid, refresh_token: &str) -> AppResult<()> {
        sqlx::query(
            r#"
            INSERT INTO refresh_tokens (user_id, token, expires_at)
            VALUES ($1, $2, NOW() + INTERVAL '7 days')
            ON CONFLICT (user_id)
            DO UPDATE SET
                token = EXCLUDED.token,
                expires_at = EXCLUDED.expires_at,
                updated_at = NOW()
        "#,
        )
        .bind(user_id)
        .bind(refresh_token)
        .execute(&self.pool)
        .await?;

        Ok(())
    }
}
