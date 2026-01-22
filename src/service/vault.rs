use async_trait::async_trait;
use uuid::Uuid;

use crate::model::app_error::AppResult;

#[async_trait]
pub trait VaultPersistence: Send + Sync {
    async fn insert(
        &self,
        user_id: Uuid,
        title: &str,
        encrypted_data: &str,
        nonce: &str,
        item_type: &str,
    ) -> AppResult<()>;
}
