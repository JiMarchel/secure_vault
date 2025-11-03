use async_trait::async_trait;
use uuid::Uuid;

use crate::model::{app_error::AppResult, user::User};

#[async_trait]
pub trait UserPersistence: Send + Sync {
    async fn create_user(&self, username: &str, email: &str) -> AppResult<Uuid>;
    async fn get_user_by_email(&self, email: &str) -> AppResult<Option<User>>;
    async fn get_user_by_id(&self, id: Uuid)-> AppResult<Option<User>>;
}
