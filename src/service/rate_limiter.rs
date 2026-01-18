use std::sync::Arc;

use redis::AsyncCommands;
use tracing::{instrument, warn};
use uuid::Uuid;

use crate::{
    model::app_error::{AppError, AppResult},
    service::email::{EmailPayload, EmailService, EmailTemplate},
};

// 10 Minutes
const LOCK_DURATION_SECS: u64 = 600;
const ATTEMPTS_EXPIRY_SECS: u64 = 600;

pub struct RateLimiterService {
    redis: redis::aio::ConnectionManager,
    email_service: Arc<dyn EmailService>,
}

pub enum RateLimit {
    Allowed { remaining: u32 },
    Locked { retry_after: i64 },
}

impl RateLimiterService {
    pub fn new(redis: redis::aio::ConnectionManager, email_service: Arc<dyn EmailService>) -> Self {
        Self {
            redis,
            email_service,
        }
    }

    fn attempts_key(email: &str) -> String {
        format!("rate_limit:login_attempts:{}", email)
    }

    fn lock_key(email: &str) -> String {
        format!("rate_limit:login_lock:{}", email)
    }

    fn unlock_token_key(token: &str) -> String {
        format!("unlock_token:{}", token)
    }

    #[instrument(name = "rate_limiter.is_locked", skip(self), fields(email = %email))]
    pub async fn is_locked(&self, email: &str) -> AppResult<Option<i64>> {
        let lock_key = Self::lock_key(email);
        let is_blocked: bool = self.redis.clone().exists(&lock_key).await?;

        if is_blocked {
            let ttl: i64 = self.redis.clone().ttl(&lock_key).await?;
            return Ok(Some(ttl));
        }

        Ok(None)
    }

    /// Record a failed login attempt. Increments counter and locks account if max attempts reached.
    /// Sends notification email when account is locked.
    #[instrument(
        name = "rate_limiter.record_failed_attempt",
        skip(self),
        fields(email = %email, max_attempts = %max_attempts)
    )]
    pub async fn record_failed_attempt(
        &self,
        email: &str,
        username: &str,
        max_attempts: u32,
    ) -> AppResult<RateLimit> {
        let attempts_key = Self::attempts_key(email);
        let lock_key = Self::lock_key(email);

        let new_count: u32 = self.redis.clone().incr(&attempts_key, 1).await?;
        self.redis
            .clone()
            .expire::<_, ()>(&attempts_key, ATTEMPTS_EXPIRY_SECS as i64)
            .await?;

        if new_count >= max_attempts {
            self.redis
                .clone()
                .set_ex::<_, _, ()>(&lock_key, "locked", LOCK_DURATION_SECS)
                .await?;

            self.redis.clone().del::<_, ()>(&attempts_key).await?;

            let unlock_token = self.generate_unlock_token(email).await?;

            let email_payload = EmailPayload {
                to_email: email.to_string(),
                to_username: username.to_string(),
                template: EmailTemplate::AccountLocked {
                    unlock_token,
                    expires_in: LOCK_DURATION_SECS / 60,
                },
            };

            if let Err(e) = self.email_service.send_async(email_payload).await {
                warn!(error = %e, "Failed to send account locked notification email");
            }

            return Ok(RateLimit::Locked {
                retry_after: LOCK_DURATION_SECS as i64,
            });
        }

        Ok(RateLimit::Allowed {
            remaining: max_attempts - new_count,
        })
    }

    #[instrument(name = "rate_limiter.clear_attempts", skip(self), fields(email = %email))]
    pub async fn clear_attempts(&self, email: &str) -> AppResult<()> {
        let attempts_key = Self::attempts_key(email);
        self.redis.clone().del::<_, ()>(&attempts_key).await?;
        Ok(())
    }

    #[instrument(name = "unlock_token.generate_unlock_token", skip(self), fields(email = %email))]
    pub async fn generate_unlock_token(&self, email: &str) -> AppResult<String> {
        let token = Uuid::new_v4().to_string();
        let key = Self::unlock_token_key(&token);

        self.redis
            .clone()
            .set_ex::<_, _, ()>(&key, email, LOCK_DURATION_SECS)
            .await?;

        Ok(token)
    }

    #[instrument(name = "unlock_token.unlock_with_token", skip(self, token))]
    pub async fn unlock_with_token(&self, token: &str) -> AppResult<String> {
        let key = Self::unlock_token_key(token);

        let email: Option<String> = self.redis.clone().get(&key).await?;

        if let Some(email) = email {
            let lock_key = Self::lock_key(&email);
            self.redis.clone().del::<_, ()>(&lock_key).await?;

            let attempts_key = Self::attempts_key(&email);
            self.redis.clone().del::<_, ()>(&attempts_key).await?;

            self.redis.clone().del::<_, ()>(&key).await?;

            Ok(email)
        } else {
            Err(AppError::Redis("Invalid token".to_string()))
        }
    }
}
