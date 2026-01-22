use async_trait::async_trait;
use redis::AsyncCommands;
use tracing::instrument;
use uuid::Uuid;

use crate::model::app_error::AppResult;

use super::RedisPersistence;

#[async_trait]
pub trait TokenStorePersistence: Send + Sync {
    async fn insert(
        &self,
        token_type: &str,
        token: &str,
        value: &str,
        ttl_secs: u64,
    ) -> AppResult<()>;
    async fn find(&self, token_type: &str, token: &str) -> AppResult<Option<String>>;
    async fn delete(&self, token_type: &str, token: &str) -> AppResult<()>;
    async fn generate_and_store(
        &self,
        token_type: &str,
        value: &str,
        ttl_secs: u64,
    ) -> AppResult<String>;
}

#[async_trait]
impl TokenStorePersistence for RedisPersistence {
    #[instrument(name = "redis.token_store.insert", skip(self, value))]
    async fn insert(
        &self,
        token_type: &str,
        token: &str,
        value: &str,
        ttl_secs: u64,
    ) -> AppResult<()> {
        let key = format!("token:{}:{}", token_type, token);
        self.conn()
            .set_ex::<_, _, ()>(&key, value, ttl_secs)
            .await?;
        Ok(())
    }

    #[instrument(name = "redis.token.find", skip(self))]
    async fn find(&self, token_type: &str, token: &str) -> AppResult<Option<String>> {
        let key = format!("token:{}:{}", token_type, token);
        let value: Option<String> = self.conn().get(&key).await?;
        Ok(value)
    }

    #[instrument(name = "redis.token.delete", skip(self))]
    async fn delete(&self, token_type: &str, token: &str) -> AppResult<()> {
        let key = format!("token:{}:{}", token_type, token);
        self.conn().del::<_, ()>(&key).await?;
        Ok(())
    }

    #[instrument(name = "redis.token.generate", skip(self, value))]
    async fn generate_and_store(
        &self,
        token_type: &str,
        value: &str,
        ttl_secs: u64,
    ) -> AppResult<String> {
        let token = Uuid::new_v4().to_string();
        self.insert(token_type, &token, value, ttl_secs)
            .await?;
        Ok(token)
    }
}
