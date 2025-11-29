use async_trait::async_trait;
use tracing::instrument;
use uuid::Uuid;

use crate::{
    model::{app_error::AppResult, user::User},
    persistence::postgres::PostgresPersistence,
    service::user::UserPersistence,
};

#[async_trait]
impl UserPersistence for PostgresPersistence {
    #[instrument(
        name= "persistence.create_user",
        skip(self),
        fields(email=%email, username=%username)
    )]
    async fn create_user(&self, username: &str, email: &str) -> AppResult<Uuid> {
        Ok(
            sqlx::query_scalar("INSERT INTO users (username, email) VALUES($1, $2) RETURNING id")
                .bind(username)
                .bind(email)
                .fetch_one(&self.pool)
                .await?,
        )
    }

    #[instrument(
        name= "persistence.get_user_by_email",
        skip(self),
        fields(email=%email)
    )]
    async fn get_user_by_email(&self, email: &str) -> AppResult<Option<User>> {
        Ok(
            sqlx::query_as::<_, User>("SELECT * FROM users WHERE email = $1")
                .bind(email)
                .fetch_optional(&self.pool)
                .await?,
        )
    }

    #[instrument(
        name= "persistence.get_user_by_id",
        skip(self),
        fields(user_id=%id)
    )]
    async fn get_user_by_id(&self, id: Uuid) -> AppResult<Option<User>> {
        Ok(
            sqlx::query_as::<_, User>("SELECT * FROM users WHERE id = $1")
                .bind(id)
                .fetch_optional(&self.pool)
                .await?,
        )
    }

    #[instrument(
        name= "persistence.update_email_verification",
        skip(self),
        fields(user_id=%id)
    )]
    async fn update_email_verification(&self, id: Uuid) -> AppResult<()> {
        sqlx::query("UPDATE users SET is_email_verified = true WHERE id = $1")
            .bind(id)
            .execute(&self.pool)
            .await?;

        Ok(())
    }

    #[instrument(
        name= "persistence.update_user_identifier",
        skip(self, encrypted_dek, nonce, salt, argon2_params),
        fields(user_id=%user_id)
    )]
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
        .await?;

        Ok(())
    }
}
