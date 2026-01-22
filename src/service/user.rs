use async_trait::async_trait;
use uuid::Uuid;

use crate::model::{
    app_error::AppResult,
    user::{User, PublicUser, UserIdentifier},
};

#[async_trait]
pub trait UserPersistence: Send + Sync {
    async fn insert(&self, username: &str, email: &str) -> AppResult<Uuid>;
    async fn find_by_email(&self, email: &str) -> AppResult<Option<User>>;
    async fn find_by_id(&self, id: Uuid) -> AppResult<Option<User>>;
    async fn update_email_verified_by_id(&self, id: Uuid) -> AppResult<()>;
    async fn update_identifier_by_id(
        &self,
        encrypted_dek: String,
        nonce: String,
        salt: String,
        argon2_params: String,
        auth_verifier: String,
        user_id: Uuid,
    ) -> AppResult<()>;
    async fn find_verifier_by_email(&self, email: &str) -> AppResult<Option<String>>;
    async fn find_identifier_by_email(&self, email: &str) -> AppResult<Option<UserIdentifier>>;
    async fn find_public_user_by_email(&self, email: &str) -> AppResult<Option<PublicUser>>;
    async fn find_public_user_by_id(&self, id: Uuid) -> AppResult<Option<PublicUser>>;
}
