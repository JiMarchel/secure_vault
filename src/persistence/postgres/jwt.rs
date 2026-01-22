use async_trait::async_trait;
use sqlx::Row;
use tracing::instrument;
use uuid::Uuid;

use crate::{
    model::{app_error::AppResult, jwt::StoredRefreshToken},
    persistence::postgres::PostgresPersistence,
    service::jwt::JwtPersistence,
};

#[async_trait]
impl JwtPersistence for PostgresPersistence {
    #[instrument(
        name = "persistence.jwt.insert_rt",
        skip(self, token, user_id, token_family)
    )]
    async fn insert_rt(
        &self,
        user_id: Uuid,
        token: &str,
        token_family: Uuid,
    ) -> AppResult<()> {
        sqlx::query(
            r#"
            INSERT INTO refresh_tokens (user_id, token, token_family, expires_at)
            VALUES ($1, $2, $3, NOW() + INTERVAL '7 days')
            ON CONFLICT (user_id)
            DO UPDATE SET
                token = EXCLUDED.token,
                token_family = EXCLUDED.token_family,
                expires_at = EXCLUDED.expires_at,
                is_revoked = FALSE,
                updated_at = NOW()
        "#,
        )
        .bind(user_id)
        .bind(token)
        .bind(token_family)
        .execute(&self.pool)
        .await?;

        Ok(())
    }

    #[instrument(name = "persistence.jwt.find_rt_by_id", skip(self, user_id))]
    async fn find_rt_by_id(&self, user_id: Uuid) -> AppResult<Option<StoredRefreshToken>> {
        let row = sqlx::query(
            r#"
            SELECT token, token_family, is_revoked
            FROM refresh_tokens 
            WHERE user_id = $1 AND expires_at > NOW()
            "#,
        )
        .bind(user_id)
        .fetch_optional(&self.pool)
        .await?;

        Ok(row.map(|r| StoredRefreshToken {
            token: r.get("token"),
            token_family: r.get("token_family"),
            is_revoked: r.get("is_revoked"),
        }))
    }

    #[instrument(name = "persistence.jwt.revoke_token_family_by_id", skip(self, user_id))]
    async fn revoke_token_family_by_id(&self, user_id: Uuid) -> AppResult<()> {
        sqlx::query(
            r#"
            UPDATE refresh_tokens 
            SET is_revoked = TRUE, updated_at = NOW()
            WHERE user_id = $1
            "#,
        )
        .bind(user_id)
        .execute(&self.pool)
        .await?;

        Ok(())
    }

    #[instrument(name = "persistence.jwt.delete_rt_by_id", skip(self, user_id))]
    async fn delete_rt_by_id(&self, user_id: Uuid) -> AppResult<()> {
        sqlx::query("DELETE FROM refresh_tokens WHERE user_id = $1")
            .bind(user_id)
            .execute(&self.pool)
            .await?;

        Ok(())
    }
}
