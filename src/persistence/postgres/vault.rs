use crate::{
    model::app_error::AppResult, persistence::postgres::PostgresPersistence,
    service::vault::VaultPersistence,
};
use async_trait::async_trait;
use tracing::instrument;
use uuid::Uuid;

#[async_trait]
impl VaultPersistence for PostgresPersistence {
    #[instrument(
        name= "persistence.vault.insert",
        skip(self, title, encrypted_data, item_type),
        fields(user_id=%user_id)
    )]
    async fn insert(
        &self,
        user_id: Uuid,
        title: &str,
        encrypted_data: &str,
        nonce: &str,
        item_type: &str,
    ) -> AppResult<()> {
        sqlx::query(
            "INSERT INTO vaults (user_id, title, encrypted_data, nonce, item_type) VALUES ($1, $2, $3, $4, $5)",
        ).bind(user_id).bind(title).bind(encrypted_data).bind(nonce).bind(item_type).execute(&self.pool).await?;

        Ok(())
    }
}
