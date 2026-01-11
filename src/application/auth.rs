use std::sync::Arc;

use constant_time_eq::constant_time_eq;
use tower_sessions::Session;
use tracing::instrument;
use uuid::Uuid;

use crate::{
    model::{
        app_error::{AppError, AppResult},
        jwt::AuthTokens,
        response::SuccessResponse,
        user::{CheckSessionResponse, User, UserInfo},
    },
    service::{
        jwt::{JwtPersistence, JwtService},
        otp::OtpService,
        session::{get_session, insert_session, remove_session},
        user::UserPersistence,
    },
    validation::user::NewUserRequest,
};

pub struct AuthUseCase {
    pub user_persistence: Arc<dyn UserPersistence>,
    pub jwt_persistence: Arc<dyn JwtPersistence>,
    pub jwt_service: Arc<JwtService>,
    pub otp_service: Arc<OtpService>,
}

impl AuthUseCase {
    pub fn new(
        user_persistence: Arc<dyn UserPersistence>,
        jwt_persistence: Arc<dyn JwtPersistence>,
        jwt_service: Arc<JwtService>,
        otp_service: Arc<OtpService>,
    ) -> Self {
        Self {
            user_persistence,
            jwt_persistence,
            jwt_service,
            otp_service,
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

        self.otp_service
            .send_verification(user_id, email, username)
            .await?;

        insert_session(session, "verif_otp", user_id).await?;
        Ok(SuccessResponse {
            data: Some(NewUserRequest {
                username: username.to_string(),
                email: email.to_string(),
            }),
            message: "created".to_string(),
        })
    }

    pub async fn login(
        &self,
        email: &str,
        auth_verifier: &str,
    ) -> AppResult<(UserInfo, AuthTokens)> {
        // Get stored verifier
        let stored_verifier = self
            .user_persistence
            .get_auth_verifier_by_email(email)
            .await?
            .ok_or(AppError::Unauthorized(
                "Wrong email or password".to_string(),
            ))?;

        // Constant-time comparison to prevent timing attacks
        if !constant_time_eq(auth_verifier.as_bytes(), stored_verifier.as_bytes()) {
            return Err(AppError::Unauthorized(
                "Wrong email or password".to_string(),
            ));
        }

        let user = self
            .user_persistence
            .get_user_info_by_email(email)
            .await?
            .ok_or(AppError::Unauthorized(
                "Wrong email or password".to_string(),
            ))?;

        let access_token = self.jwt_service.create_access_token(user.id, email)?;
        let refresh_token = self.jwt_service.create_refresh_token(user.id, email)?;

        self.jwt_persistence
            .create_refresh_token(user.id, email)
            .await?;

        Ok((
            user,
            AuthTokens {
                access_token,
                refresh_token,
            },
        ))
    }

    #[instrument(name = "use_case.handle_existing_user", skip(self, user, session))]
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
            self.otp_service
                .resend_verification(user.id, &user.email, &user.username)
                .await?;

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
        name = "use_case.verify_user_email",
        skip(self, session, otp_code, user_id)
    )]
    pub async fn verify_user_email(
        &self,
        user_id: Uuid,
        otp_code: &str,
        session: Session,
    ) -> AppResult<SuccessResponse<()>> {
        let otp_record = self.otp_service.get_otp_by_user_id(user_id).await?;

        if otp_record.otp_code != otp_code {
            return Err(AppError::Unauthorized("Invalid OTP code.".to_string()));
        }

        if chrono::Utc::now() > otp_record.otp_expires_at {
            return Err(AppError::Unauthorized("OTP code has expired.".to_string()));
        }

        self.user_persistence
            .update_email_verification(user_id)
            .await?;
        self.otp_service.delete_otp_by_user_id(user_id).await?;

        remove_session(session.clone(), "verif_otp").await?;
        insert_session(session, "verif_password", user_id).await?;

        Ok(SuccessResponse {
            data: None,
            message: "Verified Otp success!".to_string(),
        })
    }

    #[instrument(
        name = "use_case.update_user_identifier",
        skip(self, session, encrypted_dek, nonce, salt, argon2_params, auth_verifier),
        fields(user_id = %user_id)
    )]
    pub async fn update_user_identifier(
        &self,
        encrypted_dek: String,
        nonce: String,
        salt: String,
        argon2_params: String,
        auth_verifier: String,
        user_id: Uuid,
        session: Session,
    ) -> AppResult<AuthTokens> {
        self.user_persistence
            .update_user_identifier(
                encrypted_dek,
                nonce,
                salt,
                argon2_params,
                auth_verifier,
                user_id,
            )
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

        self.jwt_persistence
            .create_refresh_token(user_id, &refresh_token)
            .await?;

        Ok(AuthTokens {
            access_token,
            refresh_token,
        })
    }

    #[instrument(name = "use_case.refresh_tokens", skip(self, refresh_token))]
    pub async fn refresh_tokens(&self, refresh_token: &str) -> AppResult<AuthTokens> {
        let claims = self.jwt_service.verify_token(refresh_token)?;
        let user_id = claims.sub;

        let stored_token = self.jwt_persistence.get_refresh_token(user_id).await?;

        match stored_token {
            Some(token) if token == refresh_token => {
                let user = self
                    .user_persistence
                    .get_user_by_id(user_id)
                    .await?
                    .ok_or(AppError::NotFound("User not found".to_string()))?;

                let new_access_token =
                    self.jwt_service.create_access_token(user_id, &user.email)?;

                let new_refresh_token = self
                    .jwt_service
                    .create_refresh_token(user_id, &user.email)?;

                self.jwt_persistence
                    .create_refresh_token(user_id, &new_refresh_token)
                    .await?;

                Ok(AuthTokens {
                    access_token: new_access_token,
                    refresh_token: new_refresh_token,
                })
            }
            _ => {
                // If token doesn't match or doesn't exist, it might be reuse attempt or invalid.
                // For security, we should invalidate the existing token (if any) to prevent further abuse.
                // Does this mean we delete it?
                // The prompt says "features = [refresh token]".
                // In a stricter implementation (Reuse Detection), we would delete the token family.
                // For now, let's just return Unauthorized.
                Err(AppError::Unauthorized("Invalid refresh token".to_string()))
            }
        }
    }

    #[instrument(name = "use_case.logout_user", skip(self))]
    pub async fn logout_user(&self, user_id: Uuid) -> AppResult<SuccessResponse<()>> {
        self.jwt_persistence.delete_refresh_token(user_id).await?;

        Ok(SuccessResponse {
            data: None,
            message: "Logged out successfully".to_string(),
        })
    }

    #[instrument(name = "use_case.check_session_status", skip(self, session))]
    pub async fn check_session_status(&self, session: Session) -> AppResult<CheckSessionResponse> {
        if get_session(session.clone(), "verif_otp").await.is_ok() {
            return Ok(CheckSessionResponse {
                authenticated: false,
                state: "verif_otp".to_string(),
            });
        }

        if get_session(session, "verif_password").await.is_ok() {
            return Ok(CheckSessionResponse {
                authenticated: false,
                state: "verif_password".to_string(),
            });
        }

        Ok(CheckSessionResponse {
            authenticated: false,
            state: "".to_string(),
        })
    }
}
