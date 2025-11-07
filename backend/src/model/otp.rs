use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct VerifyOtp {
    pub otp_code: String,
}

#[derive(Serialize, sqlx::FromRow)]
pub struct OtpRecord {
    pub otp_code: String,
    pub otp_expires_at: chrono::DateTime<chrono::Utc>,
}