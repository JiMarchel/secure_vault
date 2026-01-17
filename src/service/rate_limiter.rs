use redis::AsyncCommands;

use crate::model::app_error::AppResult;

pub struct RateLimiter {
    redis: redis::aio::ConnectionManager,
}

pub enum RateLimit {
    Allowed { remaining: u32 },
    Locked { retry_after: i64 },
}

impl RateLimiter {
    pub fn new(redis: redis::aio::ConnectionManager) -> Self {
        Self { redis }
    }

    pub async fn check_email_limit(&self, email: &str, max_attempts: u32) -> AppResult<RateLimit> {
        let attempts_key = format!("rate_limit:email_attempts:{}", email);
        let lock_key = format!("rate_limit:email_lock:{}", email);

        let is_blocked = self.redis.clone().exists(&lock_key).await?;

        if is_blocked {
            let ttl: i64 = self.redis.clone().ttl(&lock_key).await?;
            return Ok(RateLimit::Locked { retry_after: ttl });
        }

        let current_attempts: Option<u32> = self.redis.clone().get(&attempts_key).await?;

        let attempts = current_attempts.unwrap_or(0);

        if attempts >= max_attempts {
            self.redis
                .clone()
                .set_ex::<_, _, ()>(&lock_key, true, 600)
                .await?;

            return Ok(RateLimit::Locked { retry_after: 600 });
        }

        Ok(RateLimit::Allowed {
            remaining: max_attempts - attempts,
        })
    }

    pub async fn increment_email_attempts(&self, email: &str) -> AppResult<()> {
        let attempts_key = format!("rate_limit:email_attempts:{}", email);
        self.redis.clone().incr::<_, _, ()>(&attempts_key, 1).await?;

        Ok(())
    }

    pub async fn check_ip_limit(
        &self,
        ip: &str,
        max_requests: u32,
        window_secs: u64,
    ) -> AppResult<bool> {
        let key = format!("rate_limit:ip:{}", &ip);

        let count: Option<u32> = self.redis.clone().get(&key).await?;

        match count {
            Some(c) if c >= max_requests => Ok(false),
            Some(_) => {
                self.redis.clone().incr::<_, _, ()>(&key, 1).await?;
                Ok(true)
            }
            None => {
                self.redis
                    .clone()
                    .set_ex::<_, _, ()>(&key, 1, window_secs)
                    .await?;
                Ok(true)
            }
        }
    }
}
