use async_trait::async_trait;
use redis::AsyncCommands;
use tracing::instrument;

use crate::{
    model::{app_error::AppResult, rate_limiter::RateLimitResult},
    persistence::redis::RedisPersistence,
};

#[async_trait]
pub trait RateLimiterPersistence: Send + Sync {
    async fn check_rate_limit(
        &self,
        key: &str,
        max_attempts: u32,
        window_secs: u64,
    ) -> AppResult<RateLimitResult>;
    async fn is_locked(&self, key: &str) -> AppResult<Option<i64>>;
    async fn lock(&self, key: &str, duration_secs: u64) -> AppResult<()>;
    async fn unlock(&self, key: &str) -> AppResult<()>;
    async fn clear_attempts(&self, key: &str) -> AppResult<()>;
}

#[async_trait]
impl RateLimiterPersistence for RedisPersistence {
    #[instrument(name = "redis.rate_limit.check", skip(self))]
    async fn check_rate_limit(
        &self,
        key: &str,
        max_attempts: u32,
        window_secs: u64,
    ) -> AppResult<RateLimitResult> {
        let attempts_key = format!("{}:attempts", key);

        let new_count: u32 = self.conn().incr(&attempts_key, 1).await?;

        if new_count == 1 {
            self.conn()
                .expire::<_, ()>(&attempts_key, window_secs as i64)
                .await?;
        }

        let allowed = new_count <= max_attempts;
        let remaining = if allowed { max_attempts - new_count } else { 0 };

        let retry_after = if !allowed {
            let ttl: i64 = self.conn().ttl(&attempts_key).await?;
            if ttl > 0 { Some(ttl as u64) } else { None }
        } else {
            None
        };

        Ok(RateLimitResult {
            allowed,
            attempts: new_count,
            remaining,
            retry_after,
        })
    }

    #[instrument(name = "redis.rate_limit.is_locked", skip(self))]
    async fn is_locked(&self, key: &str) -> AppResult<Option<i64>> {
        let lock_key = format!("{}:lock", key);
        let is_locked: bool = self.conn().exists(&lock_key).await?;

        if is_locked {
            let ttl: i64 = self.conn().ttl(&lock_key).await?;
            return Ok(Some(ttl));
        }

        Ok(None)
    }

    #[instrument(name = "redis.rate_limit.lock", skip(self))]
    async fn lock(&self, key: &str, duration_secs: u64) -> AppResult<()> {
        let lock_key = format!("{}:lock", key);
        let attempts_key = format!("{}:attempts", key);

        self.conn()
            .set_ex::<_, _, ()>(&lock_key, "locked", duration_secs)
            .await?;

        self.conn().del::<_, ()>(&attempts_key).await?;

        Ok(())
    }

    #[instrument(name = "redis.rate_limit.unlock", skip(self))]
    async fn unlock(&self, key: &str) -> AppResult<()> {
        let lock_key = format!("{}:lock", key);
        let attempts_key = format!("{}:attempts", key);

        self.conn().del::<_, ()>(&lock_key).await?;
        self.conn().del::<_, ()>(&attempts_key).await?;

        Ok(())
    }

    #[instrument(name = "redis.rate_limit.clear_attempts", skip(self))]
    async fn clear_attempts(&self, key: &str) -> AppResult<()> {
        let attempts_key = format!("{}:attempts", key);
        self.conn().del::<_, ()>(&attempts_key).await?;
        Ok(())
    }
}
