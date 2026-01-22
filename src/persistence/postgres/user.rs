use async_trait::async_trait;
use tracing::instrument;
use uuid::Uuid;

use crate::{
    model::{
        app_error::AppResult,
        user::{PublicUser, User, UserIdentifier},
    },
    persistence::postgres::PostgresPersistence,
    service::user::UserPersistence,
};

#[async_trait]
impl UserPersistence for PostgresPersistence {
    #[instrument(
        name= "persistence.user.insert",
        skip(self),
        fields(email=%email, username=%username)
    )]
    async fn insert(&self, username: &str, email: &str) -> AppResult<Uuid> {
        Ok(
            sqlx::query_scalar("INSERT INTO users (username, email) VALUES($1, $2) RETURNING id")
                .bind(username)
                .bind(email)
                .fetch_one(&self.pool)
                .await?,
        )
    }

    #[instrument(
        name= "persistence.user.find_by_email",
        skip(self),
        fields(email=%email)
    )]
    async fn find_by_email(&self, email: &str) -> AppResult<Option<User>> {
        Ok(
            sqlx::query_as::<_, User>("SELECT * FROM users WHERE email = $1")
                .bind(email)
                .fetch_optional(&self.pool)
                .await?,
        )
    }

    #[instrument(
        name= "persistence.user.find_by_id",
        skip(self),
        fields(user_id=%id)
    )]
    async fn find_by_id(&self, id: Uuid) -> AppResult<Option<User>> {
        Ok(
            sqlx::query_as::<_, User>("SELECT * FROM users WHERE id = $1")
                .bind(id)
                .fetch_optional(&self.pool)
                .await?,
        )
    }

    #[instrument(
        name= "persistence.user.update_email_verified_by_id",
        skip(self),
        fields(user_id=%id)
    )]
    async fn update_email_verified_by_id(&self, id: Uuid) -> AppResult<()> {
        sqlx::query("UPDATE users SET is_email_verified = true WHERE id = $1")
            .bind(id)
            .execute(&self.pool)
            .await?;

        Ok(())
    }

    #[instrument(
        name= "persistence.user.find_identifier_by_email",
        skip(self),
        fields(email=%email)
    )]
    async fn find_identifier_by_email(&self, email: &str) -> AppResult<Option<UserIdentifier>> {
        Ok(sqlx::query_as::<_, UserIdentifier>(
            "SELECT encrypted_dek, nonce, salt, argon2_params, auth_verifier FROM users WHERE email = $1",
        )
        .bind(email)
        .fetch_optional(&self.pool)
        .await?)
    }

    #[instrument(
        name= "persistence.user.update_identifier_by_id",
        skip(self, encrypted_dek, nonce, salt, argon2_params, auth_verifier),
        fields(user_id=%user_id)
    )]
    async fn update_identifier_by_id(
        &self,
        encrypted_dek: String,
        nonce: String,
        salt: String,
        argon2_params: String,
        auth_verifier: String,
        user_id: Uuid,
    ) -> AppResult<()> {
        sqlx::query(
            "UPDATE users SET encrypted_dek = $1, nonce = $2, salt = $3, argon2_params = $4, auth_verifier = $5 WHERE id = $6",
        )
        .bind(encrypted_dek)
        .bind(nonce)
        .bind(salt)
        .bind(argon2_params)
        .bind(auth_verifier)
        .bind(user_id)
        .execute(&self.pool)
        .await?;

        Ok(())
    }

    #[instrument(
        name= "persistence.user.find_verifier_by_email",
        skip(self),
        fields(email=%email)
    )]
    async fn find_verifier_by_email(&self, email: &str) -> AppResult<Option<String>> {
        Ok(
            sqlx::query_scalar("SELECT auth_verifier FROM users WHERE email = $1")
                .bind(email)
                .fetch_optional(&self.pool)
                .await?,
        )
    }

    #[instrument(
        name= "persistence.user.find_public_user_by_email",
        skip(self),
        fields(email=%email)
    )]
    async fn find_public_user_by_email(&self, email: &str) -> AppResult<Option<PublicUser>> {
        Ok(sqlx::query_as::<_, PublicUser>(
            "SELECT id, email, username FROM users WHERE email = $1",
        )
        .bind(email)
        .fetch_optional(&self.pool)
        .await?)
    }

    #[instrument(
        name= "persistence.user.find_public_user_by_id",
        skip(self),
        fields(user_id=%id)
    )]
    async fn find_public_user_by_id(&self, id: Uuid) -> AppResult<Option<PublicUser>> {
        Ok(
            sqlx::query_as::<_, PublicUser>("SELECT id, email, username FROM users WHERE id = $1")
                .bind(id)
                .fetch_optional(&self.pool)
                .await?,
        )
    }
}
