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
}
