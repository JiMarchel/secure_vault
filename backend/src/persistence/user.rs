use async_trait::async_trait;
use uuid::Uuid;

use crate::{
    model::{
        app_error::{AppError, AppResult},
        user::User,
    },
    persistence::postgres::PostgresPersistence,
    service::user::UserPersistence,
};

#[async_trait]
impl UserPersistence for PostgresPersistence {
    async fn create_user(&self, username: &str, email: &str) -> AppResult<Uuid> {
        let id =
            sqlx::query_scalar("INSERT INTO users (username, email) VALUES($1, $2) RETURNING id")
                .bind(username)
                .bind(email)
                .fetch_one(&self.pool)
                .await
                .map_err(AppError::from)?;

        Ok(id)
    }

    async fn get_user_by_email(&self, email: &str) -> AppResult<Option<User>> {
        let user = sqlx::query_as::<_, User>("SELECT * FROM users WHERE email = $1")
            .bind(email)
            .fetch_optional(&self.pool)
            .await
            .map_err(AppError::from)?;

        Ok(user)
    }

    async fn get_user_by_id(&self, id: Uuid) -> AppResult<Option<User>> {
        let user = sqlx::query_as::<_, User>("SELECT * FROM users WHERE id = $1")
            .bind(id)
            .fetch_optional(&self.pool)
            .await
            .map_err(AppError::from)?;

        Ok(user)
    }

    async fn update_email_verification(&self, id: Uuid) -> AppResult<()> {
        sqlx::query("UPDATE users SET is_email_verified = true WHERE id = $1")
            .bind(id)
            .execute(&self.pool)
            .await
            .map_err(AppError::from)?;

        Ok(())
    }

    async fn update_user_identifier(
        &self,
        encrypted_dek: String,
        nonce: String,
        salt: String,
        argon2_params: String,
        user_id: Uuid,
    ) -> AppResult<()> {
        sqlx::query(
            "UPDATE users SET encrypted_dek = $1, nonce = $2, salt = $3, argon2_params = $4 WHERE id = $5",
        )
        .bind(encrypted_dek)
        .bind(nonce)
        .bind(salt)
        .bind(argon2_params)
        .bind(user_id)
        .execute(&self.pool)
        .await
        .map_err(AppError::from)?;

        Ok(())
    }

    async fn save_refresh_token(&self, user_id: Uuid, refresh_token: &str) -> AppResult<()> {
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
        .await
        .map_err(AppError::from)?;

        Ok(())
    }
}
