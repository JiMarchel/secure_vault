use async_trait::async_trait;
use sqlx::Row;
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

    #[instrument(name = "persistence.get_refresh_token", skip(self, user_id))]
    async fn get_refresh_token(&self, user_id: Uuid) -> AppResult<Option<String>> {
        let row = sqlx::query(
            r#"
            SELECT token 
            FROM refresh_tokens 
            WHERE user_id = $1 AND expires_at > NOW()
            "#,
        )
        .bind(user_id)
        .fetch_optional(&self.pool)
        .await?;

        Ok(row.map(|r| r.get("token")))
    }

    #[instrument(name = "persistence.delete_refresh_token", skip(self, user_id))]
    async fn delete_refresh_token(&self, user_id: Uuid) -> AppResult<()> {
        sqlx::query("DELETE FROM refresh_tokens WHERE user_id = $1")
            .bind(user_id)
            .execute(&self.pool)
            .await?;

        Ok(())
    }
}
