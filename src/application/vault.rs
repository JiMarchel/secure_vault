use std::sync::Arc;

use tracing::instrument;
use uuid::Uuid;

use crate::{
    model::{
        app_error::AppResult,
        vault::{UpdateVaultRequest, VaultRequest, Vaults},
    },
    service::vault::VaultPersistence,
};

pub struct VaultUseCase {
    vault_persistence: Arc<dyn VaultPersistence>,
}

impl VaultUseCase {
    pub fn new(vault_persistence: Arc<dyn VaultPersistence>) -> Self {
        Self { vault_persistence }
    }

    #[instrument(name = "application.vault.create_vault", skip(self, vault), fields(user_id=%user_id))]
    pub async fn create_vault(&self, user_id: Uuid, vault: VaultRequest) -> AppResult<()> {
        self.vault_persistence
            .insert(
                user_id,
                vault.title.as_str(),
                vault.encrypted_data.as_str(),
                vault.nonce.as_str(),
                vault.item_type.string(),
            )
            .await?;

        Ok(())
    }

    #[instrument(name = "application.vault.get_all_vaults", skip(self), fields(user_id=%user_id))]
    pub async fn get_all_vaults(&self, user_id: Uuid) -> AppResult<Vec<Vaults>> {
        self.vault_persistence.find_all_by_user_id(user_id).await
    }

    #[instrument(name = "application.vault.update_vault", skip(self, vault), fields(user_id=%user_id))]
    pub async fn update_vault(&self, user_id: Uuid, vault: UpdateVaultRequest) -> AppResult<()> {
        self.vault_persistence.update(user_id, vault).await
    }

    #[instrument(name="application.vault.delete_vault", skip(self) fields(user_id=%user_id, id=%id))]
    pub async fn delete_vault(&self, user_id: Uuid, id: Uuid) -> AppResult<()> {
        self.vault_persistence.delete(user_id, id).await
    }

    #[instrument(name="application.vault.search_by_title", skip(self, title), fields(user_id=%user_id))]
    pub async fn search_by_title(&self, user_id: Uuid, title: String) -> AppResult<Vec<Vaults>> {
        self.vault_persistence.search_by_title(user_id, title).await
    }
}
