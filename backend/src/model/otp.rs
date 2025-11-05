use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct VerifyOtp {
    pub otp_code: String,
}