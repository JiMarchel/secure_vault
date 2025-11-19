use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct VerifyOtpPayload {
    pub otp_code: String,
}

#[derive(Serialize, sqlx::FromRow)]
#[serde(rename_all = "camelCase")]
pub struct OtpRecord {
    pub otp_code: String,
    pub otp_expires_at: chrono::DateTime<chrono::Utc>,
}