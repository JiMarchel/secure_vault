use async_trait::async_trait;
use chrono::{DateTime, Utc};
use redis::AsyncCommands;
use tracing::instrument;
use uuid::Uuid;

use crate::model::{app_error::AppResult, otp::OtpRecord};

use super::RedisPersistence;

#[async_trait]
pub trait OtpPersistence: Send + Sync {
    async fn create_otp(
        &self,
        user_id: Uuid,
        code: &str,
        expires_at: DateTime<Utc>,
    ) -> AppResult<()>;

    async fn get_otp(&self, user_id: Uuid) -> AppResult<Option<OtpRecord>>;

    async fn delete_otp(&self, user_id: Uuid) -> AppResult<()>;

    async fn verify_and_delete_otp(&self, user_id: Uuid, code: &str) -> AppResult<bool>;
}

#[async_trait]
impl OtpPersistence for RedisPersistence {
    #[instrument(name = "redis.otp.create", skip(self, code))]
    async fn create_otp(
        &self,
        user_id: Uuid,
        code: &str,
        expires_at: DateTime<Utc>,
    ) -> AppResult<()> {
        let key = format!("otp:{}", user_id);

        let record = OtpRecord {
            user_id,
            code: code.to_string(),
            expires_at,
            created_at: Utc::now(),
        };

        let serialized = serde_json::to_string(&record)?;

        // Calculate TTL in seconds
        let now = Utc::now();
        let ttl = (expires_at - now).num_seconds().max(0) as u64;

        self.conn
            .clone()
            .set_ex::<_, _, ()>(&key, serialized, ttl)
            .await?;

        Ok(())
    }

    #[instrument(name = "redis.otp.get", skip(self))]
    async fn get_otp(&self, user_id: Uuid) -> AppResult<Option<OtpRecord>> {
        let key = format!("otp:{}", user_id);

        let value: Option<String> = self.conn.clone().get(&key).await?;

        match value {
            Some(v) => {
                let record: OtpRecord = serde_json::from_str(&v)?;

                // Double check expiration
                if record.expires_at < Utc::now() {
                    self.delete_otp(user_id).await?;
                    return Ok(None);
                }

                Ok(Some(record))
            }
            None => Ok(None),
        }
    }

    #[instrument(name = "redis.otp.delete", skip(self))]
    async fn delete_otp(&self, user_id: Uuid) -> AppResult<()> {
        let key = format!("otp:{}", user_id);
        self.conn.clone().del::<_, ()>(&key).await?;
        Ok(())
    }

    #[instrument(name = "redis.otp.verify_and_delete", skip(self, code))]
    async fn verify_and_delete_otp(&self, user_id: Uuid, code: &str) -> AppResult<bool> {
        let record = match self.get_otp(user_id).await? {
            Some(r) => r,
            None => return Ok(false),
        };

        let is_valid = record.code == code && record.expires_at > Utc::now();

        if is_valid {
            self.delete_otp(user_id).await?;
        }

        Ok(is_valid)
    }
}
