use std::sync::Arc;

use tower_sessions::Session;
use uuid::Uuid;

use crate::{
    model::{
        app_error::{AppError, AppResult},
        user::{SignUpResponse, User},
    },
    service::{
        email::EmailService,
        otp::{OtpGenerator, OtpPersistence},
        user::UserPersistence,
    },
};

pub struct UserUseCase {
    pub user_persistence: Arc<dyn UserPersistence>,
    pub otp_persistence: Arc<dyn OtpPersistence>,
    pub otp_generator: Arc<dyn OtpGenerator>,
    pub email_service: Arc<dyn EmailService>,
}

impl UserUseCase {
    pub fn new(
        user_persistence: Arc<dyn UserPersistence>,
        otp_persistence: Arc<dyn OtpPersistence>,
        otp_generator: Arc<dyn OtpGenerator>,
        email_service: Arc<dyn EmailService>,
    ) -> Self {
        Self {
            user_persistence,
            otp_persistence,
            otp_generator,
            email_service,
        }
    }

    pub async fn sign_up(
        &self,
        username: &str,
        email: &str,
        session: Session,
    ) -> AppResult<SignUpResponse> {
        if let Some(user_exists) = self.user_persistence.get_user_by_email(email).await? {
            return self.handle_existing_user(user_exists, session).await;
        }

        let user_id = self.user_persistence.create_user(username, email).await?;

        self.send_verification_otp(user_id, email, username).await?;
        session
            .insert("verif_otp", user_id)
            .await
            .map_err(|e| AppError::Internal(e.to_string()))?;
        Ok(SignUpResponse {
            message: "created".to_string(),
        })
    }

    async fn handle_existing_user(
        &self,
        user: User,
        session: Session,
    ) -> AppResult<SignUpResponse> {
        if user.is_pending_otp_verification() {
            session
                .insert("verif_otp", user.id)
                .await
                .map_err(|e| AppError::Internal(e.to_string()))?;
            
            return Ok(SignUpResponse {
                message: "verif_otp".to_string(),
            });
        }

        if user.is_pending_password_verification() {
            let _: Option<Uuid> = session
                .remove("verif_otp")
                .await
                .map_err(|e| AppError::Internal(e.to_string()))?;

            session
                .insert("verif_password", user.id)
                .await
                .map_err(|e| AppError::Internal(e.to_string()))?;

            return Ok(SignUpResponse {
                message: "verif_password".to_string(),
            });
        }

        Err(AppError::Conflict(
            "Email already exists, Please login.".to_string(),
        ))
    }

    pub async fn send_verification_otp(
        &self,
        user_id: Uuid,
        email: &str,
        username: &str,
    ) -> AppResult<()> {
        let otp_code = self.otp_generator.generate_otp();
        let expires_at = chrono::Utc::now() + chrono::Duration::minutes(10);

        self.otp_persistence
            .create_otp(user_id, &otp_code, expires_at)
            .await?;

        self.email_service
            .send_otp_email(email, username, &otp_code)
            .await?;

        Ok(())
    }

    pub async fn resend_verification_otp(
        &self,
        user_id: Uuid,
        email: &str,
        username: &str,
    ) -> AppResult<()> {
        let otp_code = self.otp_generator.generate_otp();
        let expires_at = chrono::Utc::now() + chrono::Duration::minutes(10);

        self.otp_persistence
            .update_otp_by_user_id(user_id, &otp_code, expires_at)
            .await?;

        self.email_service
            .send_otp_email(email, username, &otp_code)
            .await?;

        Ok(())
    }
}
