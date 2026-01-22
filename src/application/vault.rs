use std::sync::Arc;

use crate::{model::vault::VaultRequest, service::vault::VaultPersistence};

pub struct VaultUseCase {
    vault_persistence: Arc<dyn VaultPersistence>,
}

impl VaultUseCase {
    pub fn new(&self, vault_persistence: Arc<dyn VaultPersistence>) -> Self {
        Self { vault_persistence }
    }

    pub async fn create_vault(&self, vault: VaultRequest) {
        // self.vault_persistence.insert(user_id, title, encrypted_data, nonce, item_type)
        todo!()
    }
}
