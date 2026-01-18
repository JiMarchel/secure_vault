use serde::{Deserialize, Serialize};

#[derive(Serialize)]
pub struct CheckSessionResponse {
    pub authenticated: bool,
    pub state: String,
}

#[derive(Serialize, sqlx::FromRow)]
#[serde(rename_all = "camelCase")]
pub struct User {
    pub id: uuid::Uuid,
    pub username: String,
    pub email: String,
    pub encrypted_dek: Option<String>,
    pub salt: Option<String>,
    pub argon2_params: Option<String>,
    pub is_email_verified: bool,
    pub nonce: Option<String>,
    pub auth_verifier: Option<String>,
    pub created_at: chrono::NaiveDateTime,
}

#[derive(Serialize, sqlx::FromRow)]
#[serde(rename_all = "camelCase")]
pub struct UserInfo {
    pub id: uuid::Uuid,
    pub username: String,
    pub email: String,
}

#[derive(Serialize, Deserialize, sqlx::FromRow)]
#[serde(rename_all = "camelCase")]
pub struct UserIdentifier {
    pub encrypted_dek: String,
    pub salt: String,
    pub nonce: String,
    pub argon2_params: String,
    pub auth_verifier: String,
}

#[derive(Debug, Deserialize)]
pub struct UnlockAccount {
    pub token: String,
}


impl User {
    pub fn is_pending_password_verification(&self) -> bool {
        self.is_email_verified && self.encrypted_dek.is_none()
    }

    pub fn is_pending_otp_verification(&self) -> bool {
        !self.is_email_verified
    }
}
