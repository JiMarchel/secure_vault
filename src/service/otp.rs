use std::sync::Arc;

use tracing::{instrument, warn};
use uuid::Uuid;

use crate::{
    model::app_error::{AppError, AppResult},
    persistence::redis::{otp::OtpPersistence, rate_limiter::RateLimiterPersistence},
    service::email::{EmailPayload, EmailService, EmailTemplate},
};

const OTP_SEND_MAX_ATTEMPTS: u32 = 3; // Max 3 OTP sends
const OTP_SEND_WINDOW_SECS: u64 = 300; // per 5 minutes
const OTP_VERIFY_MAX_ATTEMPTS: u32 = 5; // Max 5 verification attempts
const OTP_VERIFY_WINDOW_SECS: u64 = 600; // per 10 minutes
const OTP_VALIDITY_MINUTES: i64 = 10; // OTP valid for 10 minutes
const OTP_RESEND_COOLDOWN_SECS: u64 = 60; // 1 minute cooldown between resends

fn generate_otp() -> String {
    use rand::Rng;
    let mut rng = rand::rng();
    format!("{:06}", rng.random_range(0..1_000_000))
}

pub struct OtpService {
    otp_persistence: Arc<dyn OtpPersistence>,
    rate_limiter: Arc<dyn RateLimiterPersistence>,
    email_service: Arc<dyn EmailService>,
}

impl OtpService {
    pub fn new(
        otp_persistence: Arc<dyn OtpPersistence>,
        rate_limiter: Arc<dyn RateLimiterPersistence>,
        email_service: Arc<dyn EmailService>,
    ) -> Self {
        Self {
            otp_persistence,
            rate_limiter,
            email_service,
        }
    }

    fn send_rate_limit_key(user_id: Uuid) -> String {
        format!("rate_limit:otp:send:{}", user_id)
    }

    fn verify_rate_limit_key(user_id: Uuid) -> String {
        format!("rate_limit:otp:verify:{}", user_id)
    }

    fn resend_cooldown_key(user_id: Uuid) -> String {
        format!("rate_limit:otp:resend_cooldown:{}", user_id)
    }

    #[instrument(name = "service.otp.send_verification", skip(self, email))]
    pub async fn send_verification(
        &self,
        user_id: Uuid,
        email: &str,
        username: &str,
    ) -> AppResult<()> {
        // Check send rate limit
        let send_key = Self::send_rate_limit_key(user_id);
        let rate_limit = self
            .rate_limiter
            .check_rate_limit(&send_key, OTP_SEND_MAX_ATTEMPTS, OTP_SEND_WINDOW_SECS)
            .await?;

        if !rate_limit.allowed {
            return Err(AppError::TooManyRequests {
                message: format!(
                    "Too many OTP requests. Please try again in {} seconds",
                    OTP_SEND_WINDOW_SECS
                ),
                retry_after: Some(OTP_SEND_WINDOW_SECS),
            });
        }

        // Generate and send OTP
        let otp_code = generate_otp();
        let expires_at = chrono::Utc::now() + chrono::Duration::minutes(OTP_VALIDITY_MINUTES);

        self.otp_persistence
            .insert(user_id, &otp_code, expires_at)
            .await?;

        let email_payload = EmailPayload {
            to_email: email.to_string(),
            to_username: username.to_string(),
            template: EmailTemplate::Otp {
                otp_code: otp_code.clone(),
            },
        };

        if let Err(e) = self.email_service.send_async(email_payload).await {
            warn!(error = %e, "Failed to send OTP email");
            self.otp_persistence.delete_by_id(user_id).await?;
            return Err(e);
        }

        Ok(())
    }

    #[instrument(name = "service.otp.resend_verification", skip(self, email))]
    pub async fn resend_verification(
        &self,
        user_id: Uuid,
        email: &str,
        username: &str,
    ) -> AppResult<()> {
        // Check resend cooldown
        let cooldown_key = Self::resend_cooldown_key(user_id);
        if let Some(ttl) = self.rate_limiter.is_locked(&cooldown_key).await? {
            return Err(AppError::TooManyRequests {
                message: format!("Please wait {} seconds before requesting another OTP", ttl),
                retry_after: Some(ttl as u64),
            });
        }

        // Check send rate limit
        let send_key = Self::send_rate_limit_key(user_id);
        let rate_limit = self
            .rate_limiter
            .check_rate_limit(&send_key, OTP_SEND_MAX_ATTEMPTS, OTP_SEND_WINDOW_SECS)
            .await?;

        if !rate_limit.allowed {
            return Err(AppError::TooManyRequests {
                message: format!(
                    "Too many OTP requests. Please try again in {} seconds",
                    OTP_SEND_WINDOW_SECS
                ),
                retry_after: Some(OTP_SEND_WINDOW_SECS),
            });
        }

        // Set cooldown for next resend
        self.rate_limiter
            .lock(&cooldown_key, OTP_RESEND_COOLDOWN_SECS)
            .await?;

        // Generate and send new OTP
        let otp_code = generate_otp();
        let expires_at = chrono::Utc::now() + chrono::Duration::minutes(OTP_VALIDITY_MINUTES);

        self.otp_persistence
            .insert(user_id, &otp_code, expires_at)
            .await?;

        let email_payload = EmailPayload {
            to_email: email.to_string(),
            to_username: username.to_string(),
            template: EmailTemplate::Otp {
                otp_code: otp_code.clone(),
            },
        };

        if let Err(e) = self.email_service.send_async(email_payload).await {
            warn!(error = %e, "Failed to send OTP email");
            self.otp_persistence.delete_by_id(user_id).await?;
            return Err(e);
        }

        Ok(())
    }

    #[instrument(name = "service.otp.verify", skip(self, code))]
    pub async fn verify(&self, user_id: Uuid, code: &str) -> AppResult<()> {
        // Check verify rate limit
        let verify_key = Self::verify_rate_limit_key(user_id);
        let rate_limit = self
            .rate_limiter
            .check_rate_limit(&verify_key, OTP_VERIFY_MAX_ATTEMPTS, OTP_VERIFY_WINDOW_SECS)
            .await?;

        if !rate_limit.allowed {
            return Err(AppError::TooManyRequests {
                message: format!(
                    "Too many verification attempts. Please try again in {} seconds",
                    rate_limit.retry_after.unwrap_or(OTP_VERIFY_WINDOW_SECS)
                ),
                retry_after: rate_limit.retry_after,
            });
        }

        // Verify OTP
        let is_valid = self
            .otp_persistence
            .verify_and_delete_by_id(user_id, code)
            .await?;

        if !is_valid {
            return Err(crate::model::app_error::AppError::BadRequest(
                "Invalid or expired OTP code".to_string(),
            ));
        }

        self.rate_limiter.clear_attempts(&verify_key).await?;
        
        let send_key = Self::send_rate_limit_key(user_id);
        self.rate_limiter.clear_attempts(&send_key).await?;

        Ok(())
    }
}
