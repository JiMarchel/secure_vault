use chrono::{NaiveDateTime};
use serde::{Deserialize, Serialize};
use sqlx::prelude::FromRow;
use uuid::Uuid;

#[derive(Serialize, FromRow)]
#[serde(rename_all = "camelCase")]
pub struct Vaults {
    pub id: Uuid,
    pub user_id: Uuid,
    pub title: String,
    pub item_type: ItemType,
    pub encrypted_data: String,
    pub nonce: String,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UpdateVaultRequest {
    pub id: Uuid,
    pub title: String,
    pub item_type: ItemType,
    pub encrypted_data: String,
    pub nonce: String,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct VaultRequest {
    pub title: String,
    pub item_type: ItemType,
    pub encrypted_data: String,
    pub nonce: String,
}

#[derive(Deserialize, Serialize, sqlx::Type)]
#[sqlx(type_name = "VARCHAR", rename_all = "PascalCase")]
pub enum ItemType {
    Password,
    CreditCard,
    Note,
    Contact,
}

impl ItemType {
    pub fn string(&self) -> &str {
        match self {
            ItemType::Password => "Password",
            ItemType::CreditCard => "CreditCard",
            ItemType::Note => "Note",
            ItemType::Contact => "Contact",
        }
    }
}
