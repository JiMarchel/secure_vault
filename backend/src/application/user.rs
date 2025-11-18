use std::sync::Arc;

use tower_sessions::Session;
use tracing::instrument;
use uuid::Uuid;

use crate::{
    model::{
        app_error::{AppError, AppResult}, jwt::AuthTokens, response::SuccessResponse, user::{ User}
    },
    service::{
        email::EmailService,
        jwt::JwtService,
        otp::{OtpGenerator, OtpPersistence},
        session::{insert_session, remove_session},
        user::UserPersistence,
    }, validation::user::{ NewUserRequest},
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

    #[instrument(
        name= "use_case.sign_up", 
        skip(self, session, username, email), 
        fields(email=%email, username=%username)
    )]
    pub async fn sign_up(
        &self,
        username: &str,
        email: &str,
        session: Session,
    ) -> AppResult<SuccessResponse<NewUserRequest>> {
        if let Some(user_exists) = self.user_persistence.get_user_by_email(email).await? {
            return self.handle_existing_user(user_exists, session).await;
        }

        let user_id = self.user_persistence.create_user(username, email).await?;

        self.send_verification_otp(user_id, email, username).await?;

        insert_session(session, "verif_otp", user_id).await?;
        Ok(SuccessResponse {
            data: Some(NewUserRequest { username: username.to_string(), email: email.to_string() }),
            message: "created".to_string(),
        })
    }

    #[instrument(
        name="use_case.handle_existing_user", 
        skip(self, user, session), 
        fields(user_id=%user.id, email=%user.email)
    )]
    async fn handle_existing_user(
        &self,
        user: User,
        session: Session,
    ) -> AppResult<SuccessResponse<NewUserRequest>> {
        let user_request = NewUserRequest {
            username: user.username.clone(),
            email: user.email.clone(),
        };

        if user.is_pending_otp_verification() {
            insert_session(session, "verif_otp", user.id).await?;

            return Ok(SuccessResponse {
                data: Some(user_request),
                message: "verif_otp".to_string(),
            });
        }

        if user.is_pending_password_verification() {
            let _ = remove_session(session.clone(), "verif_otp").await?;

            insert_session(session, "verif_password", user.id).await?;

            return Ok(SuccessResponse {
                data: Some(user_request),
                message: "verif_password".to_string(),
            });
        }

        Err(AppError::Conflict(
            "Email already exists, Please login.".to_string(),
        ))
    }

    #[instrument(
        name="use_case.send_verification_otp", 
        skip(self), 
        fields(user_id=%user_id, email=%email, username=%username)
    )]
    async fn send_verification_otp(
        &self,
        user_id: Uuid,
        email: &str,
        username: &str,
    ) -> AppResult<SuccessResponse<()>> {
        let otp_code = self.otp_generator.generate_otp();
        let expires_at = chrono::Utc::now() + chrono::Duration::minutes(10);

        self.otp_persistence
            .create_otp(user_id, &otp_code, expires_at)
            .await?;

        self.email_service
            .send_email_async(email, username, &otp_code).await?;

        Ok(SuccessResponse {
            data: None,
            message: "OTP sent".to_string(),
        })
    }

    #[instrument(
        name="use_case.resend_verification_otp", 
        skip(self), 
        fields(user_id=%user_id, email=%email, username=%username)
    )]
    pub async fn resend_verification_otp(
        &self,
        user_id: Uuid,
        email: &str,
        username: &str,
    ) -> AppResult<SuccessResponse<()>> {
        let otp_code = self.otp_generator.generate_otp();
        let expires_at = chrono::Utc::now() + chrono::Duration::minutes(10);

        self.otp_persistence
            .update_otp_by_user_id(user_id, &otp_code, expires_at)
            .await?;

        self.email_service
            .send_otp_email(email, username, &otp_code)
            .await?;

        Ok(SuccessResponse {
            data: None,
            message: "OTP resent".to_string(),
        })
    }

    #[instrument(
        name= "use_case.verify_user_email", 
        skip(self, session), 
        fields(user_id=%user_id, otp_length=%otp_code.len())
    )]
    pub async fn verify_user_email(
        &self,
        user_id: Uuid,
        otp_code: &str,
        session: Session,
    ) -> AppResult<SuccessResponse<()>> {
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

        Ok(SuccessResponse {
            data: None,
            message: "email_verified".to_string(),
        })
    }

    #[instrument(
        name= "use_case.update_user_identifier", 
        skip(self, session, encrypted_dek, nonce, salt, argon2_params), 
        fields(user_id=%user_id)
    )]
    pub async fn update_user_identifier(
        &self,
        encrypted_dek: String,
        nonce: String,
        salt: String,
        argon2_params: String,
        user_id: Uuid,
        session: Session,
    ) -> AppResult<SuccessResponse<AuthTokens>> {
        self.user_persistence
            .update_user_identifier(encrypted_dek, nonce, salt, argon2_params, user_id)
            .await?;

        remove_session(session, "verif_password").await?;

        let user = self
            .user_persistence
            .get_user_by_id(user_id)
            .await?
            .ok_or(AppError::NotFound("User not found".to_string()))?;

        let access_token = self.jwt_service.create_access_token(user_id, &user.email)?;
        let refresh_token = self
            .jwt_service
            .create_refresh_token(user_id, &user.email)?;

        self.user_persistence
            .save_refresh_token(user_id, &refresh_token)
            .await?;

        Ok(SuccessResponse {
            data: Some(AuthTokens {
                access_token,
                refresh_token,
                token_type: "Bearer".to_string(),
                expires_in: 900,
            }),
            message: "User identifier updated".to_string(),
        })
    }
}
