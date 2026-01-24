use crate::{
    model::{
        app_error::AppResult,
        vault::{UpdateVaultRequest, Vaults},
    },
    persistence::postgres::PostgresPersistence,
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

    #[instrument(name = "persistence.vault.find_all_by_user_id", skip(self))]
    async fn find_all_by_user_id(&self, user_id: Uuid) -> AppResult<Vec<Vaults>> {
        let rows = sqlx::query_as("SELECT * FROM vaults WHERE user_id = $1 ORDER BY title ASC")
            .bind(user_id)
            .fetch_all(&self.pool)
            .await?;

        Ok(rows)
    }

    #[instrument(name = "persistence.vault.update", skip(self, vault))]
    async fn update(&self, user_id: Uuid, vault: UpdateVaultRequest) -> AppResult<()> {
        sqlx::query(
            "UPDATE vaults SET title = $1, encrypted_data = $2, nonce = $3, item_type = $4, updated_at = NOW() WHERE id = $5 AND user_id = $6"
        ).bind(vault.title).bind(vault.encrypted_data).bind(vault.nonce).bind(vault.item_type).bind(vault.id).bind(user_id).execute(&self.pool).await?;

        Ok(())
    }

    #[instrument(name = "persistence.vault.delete", skip(self, user_id, id))]
    async fn delete(&self, user_id: Uuid, id: Uuid) -> AppResult<()> {
        sqlx::query("DELETE FROM vaults WHERE id = $1 AND user_id = $2")
            .bind(id)
            .bind(user_id)
            .execute(&self.pool)
            .await?;

        Ok(())
    }

    #[instrument(name = "persistence.vault.search_by_title", skip(self, title), fields(user_id=%user_id))]
    async fn search_by_title(&self, user_id: Uuid, title: String) -> AppResult<Vec<Vaults>> {
        let rows = sqlx::query_as(
            "SELECT * FROM vaults WHERE 
                user_id = $1
                AND LOWER(title) LIKE LOWER($2)
            ORDER BY updated_at DESC
            LIMIT 20
            ",
        )
        .bind(user_id)
        .bind(format!("%{}%", title))
        .fetch_all(&self.pool)
        .await?;

        Ok(rows)
    }
}
