use async_trait::async_trait;
use uuid::Uuid;

use crate::model::{app_error::AppResult, user::User};

#[async_trait]
pub trait UserPersistence: Send + Sync {
    async fn create_user(&self, username: &str, email: &str) -> AppResult<Uuid>;
    async fn get_user_by_email(&self, email: &str) -> AppResult<Option<User>>;
    async fn get_user_by_id(&self, id: Uuid) -> AppResult<Option<User>>;
    async fn update_email_verification(&self, id: Uuid) -> AppResult<()>;
    async fn update_user_identifier(
        &self,
        encrypted_dek: String,
        nonce: String,
        salt: String,
        argon2_params: String,
        user_id: Uuid,
    ) -> AppResult<()>;
}
