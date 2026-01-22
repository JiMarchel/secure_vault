use serde::Deserialize;

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct VaultRequest {
    pub user_id: uuid::Uuid,
    pub title: String,
    pub item_type: ItemType,
    pub encrypted_data: String,
    pub nonce: String,
}

#[derive(Deserialize)]
pub enum ItemType {
    Password,
    CreditCard,
    Note,
    Contact,
}

impl ItemType {
    pub fn string(&self) -> &str {
        match self {
            ItemType::Password => "password",
            ItemType::CreditCard => "credit-card",
            ItemType::Note => "note",
            ItemType::Contact => "contact",
        }
    }
}
