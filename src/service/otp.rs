use std::sync::Arc;

use async_trait::async_trait;
use tracing::instrument;
use uuid::Uuid;

use crate::{
    model::{app_error::AppResult, otp::OtpRecord},
    service::email::EmailService,
};

#[async_trait]
pub trait OtpPersistence: Send + Sync {
    async fn create_otp(
        &self,
        user_id: Uuid,
        code: &str,
        expires_at: chrono::DateTime<chrono::Utc>,
    ) -> AppResult<()>;
    async fn get_otp_by_user_id(&self, user_id: Uuid) -> AppResult<OtpRecord>;
    async fn update_otp_by_user_id(
        &self,
        user_id: Uuid,
        code: &str,
        expires_at: chrono::DateTime<chrono::Utc>,
    ) -> AppResult<()>;
    async fn delete_otp_by_user_id(&self, user_id: Uuid) -> AppResult<()>;
}

fn generate_otp() -> String {
    use rand::Rng;
    let mut rng = rand::rng();
    format!("{:06}", rng.random_range(0..1_000_000))
}

pub struct OtpService {
    pub otp_persistence: Arc<dyn OtpPersistence>,
    pub email_service: Arc<dyn EmailService>,
}

impl OtpService {
    pub fn new(
        otp_persistence: Arc<dyn OtpPersistence>,
        email_service: Arc<dyn EmailService>,
    ) -> Self {
        Self {
            otp_persistence,
            email_service,
        }
    }

    #[instrument(
        name = "service.send_verification_otp",
        skip(self, user_id, email, username)
    )]
    pub async fn send_verification(
        &self,
        user_id: Uuid,
        email: &str,
        username: &str,
    ) -> AppResult<()> {
        let otp_code = generate_otp();
        let expires_at = chrono::Utc::now() + chrono::Duration::minutes(10);

        self.otp_persistence
            .create_otp(user_id, &otp_code, expires_at)
            .await?;

        self.email_service
            .send_email_async(email, username, &otp_code)
            .await?;

        Ok(())
    }

    #[instrument(
        name = "service.resend_verification_otp",
        skip(self, user_id, email, username)
    )]
    pub async fn resend_verification(
        &self,
        user_id: Uuid,
        email: &str,
        username: &str,
    ) -> AppResult<()> {
        let otp_code = generate_otp();
        let expires_at = chrono::Utc::now() + chrono::Duration::minutes(10);

        self.otp_persistence
            .update_otp_by_user_id(user_id, &otp_code, expires_at)
            .await?;

        self.email_service
            .send_email_async(email, username, &otp_code)
            .await?;

        Ok(())
    }

    #[instrument(
            name = "service.get_otp_by_user_id",
        skip(self, user_id)
    )]
    pub async fn get_otp_by_user_id(&self, user_id: Uuid) -> AppResult<OtpRecord> {
        Ok(self.otp_persistence.get_otp_by_user_id(user_id).await?)
    }

    #[instrument(
        name = "service.delete_otp_by_user_id",
        skip(self, user_id)
    )]
    pub async fn delete_otp_by_user_id(&self, user_id: Uuid) -> AppResult<()> {
        self.otp_persistence.delete_otp_by_user_id(user_id).await
    }
}
