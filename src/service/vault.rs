use async_trait::async_trait;
use uuid::Uuid;

use crate::model::{
    app_error::AppResult,
    vault::{UpdateVaultRequest, Vaults},
};

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
    async fn find_all_by_user_id(&self, user_id: Uuid) -> AppResult<Vec<Vaults>>;
    async fn update(&self, user_id: Uuid, vault: UpdateVaultRequest) -> AppResult<()>;
    async fn delete(&self, user_id: Uuid, id: Uuid) -> AppResult<()>;
    async fn search_by_title(&self, user_id: Uuid, title: String) -> AppResult<Vec<Vaults>>;
}
