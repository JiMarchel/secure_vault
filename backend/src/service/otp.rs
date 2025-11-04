use async_trait::async_trait;
use uuid::Uuid;

use crate::model::{app_error::AppResult, user::OtpRecord};

#[async_trait]
pub trait OtpPersistence: Send + Sync {
    async fn create_otp(
        &self,
        user_id: Uuid,
        code: &str,
        expires_at: chrono::DateTime<chrono::Utc>,
    ) -> AppResult<()>;
    async fn get_otp_by_user_id(
        &self,
        user_id: Uuid,
    ) -> AppResult<OtpRecord>;
    async fn update_otp_by_user_id(
        &self,
        user_id: Uuid,
        code: &str,
        expires_at: chrono::DateTime<chrono::Utc>,
    ) -> AppResult<()>;
}

pub trait OtpGenerator: Send + Sync {
    fn generate_otp(&self) -> String;
}

pub struct RandomOtpGenerator;

impl OtpGenerator for RandomOtpGenerator {
    fn generate_otp(&self) -> String {
        use rand::Rng;
        let mut rng = rand::rng();
        format!("{:06}", rng.random_range(1..1_000_000))
    }
}
