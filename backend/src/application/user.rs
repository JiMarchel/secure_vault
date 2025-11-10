use std::sync::Arc;

use tower_sessions::Session;
use uuid::Uuid;

use crate::{
    model::{
        app_error::{AppError, AppResult},
        jwt::AuthTokens,
        user::{SignUpResponse, User},
    },
    service::{
        email::EmailService,
        jwt::JwtService,
        otp::{OtpGenerator, OtpPersistence},
        session::{insert_session, remove_session},
        user::UserPersistence,
    },
};

pub struct UserUseCase {
    pub user_persistence: Arc<dyn UserPersistence>,
    pub otp_persistence: Arc<dyn OtpPersistence>,
    pub otp_generator: Arc<dyn OtpGenerator>,
    pub email_service: Arc<dyn EmailService>,
    pub jwt_service: Arc<JwtService>,
}

impl UserUseCase {
    pub fn new(
        user_persistence: Arc<dyn UserPersistence>,
        otp_persistence: Arc<dyn OtpPersistence>,
        otp_generator: Arc<dyn OtpGenerator>,
        email_service: Arc<dyn EmailService>,
        jwt_service: Arc<JwtService>,
    ) -> Self {
        Self {
            user_persistence,
            otp_persistence,
            otp_generator,
            email_service,
            jwt_service,
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

        insert_session(session, "verif_otp", user_id).await?;
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
            insert_session(session, "verif_otp", user.id).await?;

            return Ok(SignUpResponse {
                message: "verif_otp".to_string(),
            });
        }

        if user.is_pending_password_verification() {
            let _ = remove_session(session.clone(), "verif_otp").await?;

            insert_session(session, "verif_password", user.id).await?;

            return Ok(SignUpResponse {
                message: "verif_password".to_string(),
            });
        }

        Err(AppError::Conflict(
            "Email already exists, Please login.".to_string(),
        ))
    }

    async fn send_verification_otp(
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

    pub async fn verify_user_email(
        &self,
        user_id: Uuid,
        otp_code: &str,
        session: Session,
    ) -> AppResult<()> {
        let otp_record = self.otp_persistence.get_otp_by_user_id(user_id).await?;

        if otp_record.otp_code != otp_code {
            return Err(AppError::Unauthorized("Invalid OTP code.".to_string()));
        }

        if chrono::Utc::now() > otp_record.otp_expires_at {
            return Err(AppError::Unauthorized("OTP code has expired.".to_string()));
        }

        self.user_persistence
            .update_email_verification(user_id)
            .await?;
        self.otp_persistence.delete_otp_by_user_id(user_id).await?;

        remove_session(session.clone(), "verif_otp").await?;
        insert_session(session, "verif_password", user_id).await?;

        Ok(())
    }

    pub async fn update_user_identifier(
        &self,
        encrypted_dek: String,
        nonce: String,
        salt: String,
        argon2_params: String,
        user_id: Uuid,
        session: Session,
    ) -> AppResult<AuthTokens> {
        self.user_persistence
            .update_user_identifier(encrypted_dek, nonce, salt, argon2_params, user_id)
            .await?;

        remove_session(session, "verif_password").await?;

        let user = self
            .user_persistence
            .get_user_by_id(user_id)
            .await?
            .ok_or(AppError::NotFound)?;

        let access_token = self.jwt_service.create_access_token(user_id, &user.email)?;
        let refresh_token = self
            .jwt_service
            .create_refresh_token(user_id, &user.email)?;

        self.user_persistence
            .save_refresh_token(user_id, &refresh_token)
            .await?;

        Ok(AuthTokens {
            access_token,
            refresh_token,
            token_type: "Bearer".to_string(),
            expires_in: 900,
        })
    }
}
