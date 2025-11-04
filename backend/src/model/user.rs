use serde::Serialize;

#[derive(Serialize)]
pub struct SignUpResponse {
    pub message: String,
}

#[derive(Serialize)]
pub struct CheckSessionResponse {
    pub authenticated: bool,
    pub message: Option<String>,
}

#[derive(Serialize, sqlx::FromRow)]
pub struct OtpRecord {
    pub otp_code: String,
    pub otp_expires_at: chrono::DateTime<chrono::Utc>,
}

#[derive(Serialize, sqlx::FromRow)]
pub struct User {
    pub id: uuid::Uuid,
    pub username: String,
    pub email: String,
    pub encrypted_dek: Option<Vec<u8>>,
    pub salt: Option<Vec<u8>>,
    pub argon2_params: Option<String>,
    pub is_email_verified: bool,
    pub nonce: Option<Vec<u8>>,
    pub created_at: chrono::NaiveDateTime,
}

impl User {
    pub fn is_pending_password_verification(&self) -> bool {
        self.is_email_verified && self.encrypted_dek.is_none()
    }

    pub fn is_pending_otp_verification(&self) -> bool {
        !self.is_email_verified
    }
}
