use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Serialize, Deserialize)]
pub struct VerifyOtpPayload {
    pub otp_code: String,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct OtpRecord {
    pub user_id: Uuid,
    pub code: String,
    pub expires_at: DateTime<Utc>,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct OtpStatusResponse {
    pub has_otp: bool,
    pub expires_at: Option<DateTime<Utc>>,
    pub can_resend: bool,
    pub resend_after: Option<u64>, // seconds until can resend
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ResendOtpResponse {
    pub success: bool,
    pub cooldown_seconds: u64,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct VerifyOtpRequest {
    pub otp_code: String,
}