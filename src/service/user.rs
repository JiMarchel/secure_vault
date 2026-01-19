use async_trait::async_trait;
use uuid::Uuid;

use crate::model::{
    app_error::AppResult,
    user::{User, UserIdentifier, UserInfo},
};

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
        auth_verifier: String,
        user_id: Uuid,
    ) -> AppResult<()>;
    async fn get_auth_verifier_by_email(&self, email: &str) -> AppResult<Option<String>>;
    async fn get_user_identifier(&self, email: &str) -> AppResult<Option<UserIdentifier>>;
    async fn get_user_info_by_email(&self, email: &str) -> AppResult<Option<UserInfo>>;
    async fn get_user_info_by_id(&self, id: Uuid) -> AppResult<Option<UserInfo>>;
}
