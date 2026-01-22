use chrono::Utc;
use std::sync::Arc;
use tracing::instrument;
use uuid::Uuid;

use crate::{
    model::{
        app_error::AppResult,
        otp::{OtpStatusResponse, ResendOtpResponse},
        response::SuccessResponse,
    },
    persistence::redis::{otp::OtpPersistence, rate_limiter::RateLimiterPersistence},
    service::{otp::OtpService, user::UserPersistence},
};

const OTP_RESEND_COOLDOWN_SECS: u64 = 60;

pub struct OtpUseCase {
    pub otp_persistence: Arc<dyn OtpPersistence>,
    pub otp_service: Arc<OtpService>,
    pub rate_limiter: Arc<dyn RateLimiterPersistence>,
    pub user_persistence: Arc<dyn UserPersistence>,
}

impl OtpUseCase {
    pub fn new(
        otp_persistence: Arc<dyn OtpPersistence>,
        otp_service: Arc<OtpService>,
        rate_limiter: Arc<dyn RateLimiterPersistence>,
        user_persistence: Arc<dyn UserPersistence>,
    ) -> Self {
        Self {
            otp_persistence,
            otp_service,
            rate_limiter,
            user_persistence,
        }
    }

    #[instrument(name = "use_case.otp.send_otp_verification", skip(self, username, email))]
    pub async fn send_otp_verification(
        &self,
        user_id: Uuid,
        username: &str,
        email: &str,
    ) -> AppResult<SuccessResponse<()>> {
        self.otp_service
            .send_verification(user_id, email, username)
            .await?;

        Ok(SuccessResponse {
            data: None,
            message: "OTP sent successfully".into(),
        })
    }

    #[instrument(name = "use_case.otp.resend_otp_verification", skip(self, username, email))]
    pub async fn resend_otp_verification(
        &self,
        user_id: Uuid,
        email: &str,
        username: &str,
    ) -> AppResult<SuccessResponse<ResendOtpResponse>> {
        self.otp_service
            .resend_verification(user_id, email, username)
            .await?;

        Ok(SuccessResponse {
            data: Some(ResendOtpResponse {
                success: true,
                cooldown_seconds: OTP_RESEND_COOLDOWN_SECS,
            }),
            message: "OTP resent successfully".into(),
        })
    }

    #[instrument(name = "use_case.otp.get_otp_status", skip(self))]
    pub async fn get_otp_status(
        &self,
        user_id: Uuid,
    ) -> AppResult<SuccessResponse<OtpStatusResponse>> {
        let otp = self.otp_persistence.find_by_id(user_id).await?;

        let (has_otp, expires_at) = match otp {
            Some(record) => {
                let is_expired = record.expires_at < Utc::now();
                if is_expired {
                    self.otp_persistence.delete_by_id(user_id).await?;
                    (false, None)
                } else {
                    (true, Some(record.expires_at))
                }
            }
            None => (false, None),
        };

        // Check if can resend (check cooldown)
        let resend_cooldown_key = format!("rate_limit:otp:resend_cooldown:{}", user_id);
        let resend_after = self.rate_limiter.is_locked(&resend_cooldown_key).await?;

        let can_resend = resend_after.is_none();

        Ok(SuccessResponse {
            data: Some(OtpStatusResponse {
                has_otp,
                expires_at,
                can_resend,
                resend_after: resend_after.map(|ttl| ttl as u64),
            }),
            message: "OTP status retrieved successfully".into(),
        })
    }

    #[instrument(name = "use_case.otp.verify_otp", skip(self, code))]
    pub async fn verify_otp(&self, user_id: Uuid, code: &str) -> AppResult<SuccessResponse<()>> {
        self.otp_service.verify(user_id, code).await?;

        self.user_persistence
            .update_email_verified_by_id(user_id)
            .await?;

        Ok(SuccessResponse {
            data: None,
            message: "OTP verified successfully".into(),
        })
    }
}
