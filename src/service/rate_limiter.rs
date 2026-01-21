use std::sync::Arc;
use tracing::{instrument, warn};

use crate::{
    model::app_error::{AppError, AppResult},
    persistence::redis::{
        rate_limiter::RateLimiterPersistence, token_store::TokenStorePersistence,
    },
    service::email::{EmailPayload, EmailService, EmailTemplate},
};

const MAX_LOGIN_ATTEMPTS: u32 = 10;
const ATTEMPTS_WINDOW_SECS: u64 = 600;
const LOCK_DURATION_SECS: u64 = 600;

pub enum LoginRateLimitStatus {
    Allowed { remaining: u32 },
    Locked { retry_after: i64 },
}

pub struct LoginRateLimiterService {
    redis: Arc<dyn RateLimiterPersistence>,
    token_store: Arc<dyn TokenStorePersistence>,
    email_service: Arc<dyn EmailService>,
}

impl LoginRateLimiterService {
    pub fn new(
        redis: Arc<dyn RateLimiterPersistence>,
        token_store: Arc<dyn TokenStorePersistence>,
        email_service: Arc<dyn EmailService>,
    ) -> Self {
        Self {
            redis,
            token_store,
            email_service,
        }
    }

    fn rate_limit_key(email: &str) -> String {
        format!("rate_limit:login:{}", email)
    }

    #[instrument(name = "login_rate_limiter.check_if_locked", skip(self))]
    pub async fn check_if_locked(&self, email: &str) -> AppResult<Option<i64>> {
        let key = Self::rate_limit_key(email);
        self.redis.is_locked(&key).await
    }

    #[instrument(name = "login_rate_limiter.record_failed_attempt", skip(self))]
    pub async fn record_failed_attempt(
        &self,
        email: &str,
        username: &str,
    ) -> AppResult<LoginRateLimitStatus> {
        let key = Self::rate_limit_key(email);

        let result = self
            .redis
            .check_rate_limit(&key, MAX_LOGIN_ATTEMPTS, ATTEMPTS_WINDOW_SECS)
            .await?;

        if !result.allowed {
            self.redis.lock(&key, LOCK_DURATION_SECS).await?;

            let unlock_token = self
                .token_store
                .generate_and_store_token("unlock", email, LOCK_DURATION_SECS)
                .await?;

            let email_payload = EmailPayload {
                to_email: email.to_string(),
                to_username: username.to_string(),
                template: EmailTemplate::AccountLocked {
                    unlock_token,
                    expires_in: LOCK_DURATION_SECS / 60,
                },
            };

            if let Err(e) = self.email_service.send_async(email_payload).await {
                warn!(error = %e, "Failed to send account locked notification");
            }

            return Ok(LoginRateLimitStatus::Locked {
                retry_after: LOCK_DURATION_SECS as i64,
            });
        }

        Ok(LoginRateLimitStatus::Allowed {
            remaining: result.remaining,
        })
    }

    #[instrument(name = "login_rate_limiter.clear_attempts", skip(self))]
    pub async fn clear_attempts(&self, email: &str) -> AppResult<()> {
        let key = Self::rate_limit_key(email);
        self.redis.clear_attempts(&key).await
    }

    #[instrument(name = "login_rate_limiter.unlock_with_token", skip(self, token))]
    pub async fn unlock_with_token(&self, token: &str) -> AppResult<String> {
        let email = self
            .token_store
            .get_token_value("unlock", token)
            .await?
            .ok_or_else(|| AppError::BadRequest("Invalid or expired unlock token".to_string()))?;

        let key = Self::rate_limit_key(&email);
        self.redis.unlock(&key).await?;

        self.token_store.delete_token("unlock", token).await?;

        Ok(email)
    }
}
