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
        user::{CheckSessionResponse, PublicUser, User},
    },
    persistence::redis::otp::OtpPersistence,
    service::{
        jwt::{JwtPersistence, JwtService},
        otp::OtpService,
        rate_limiter::LoginRateLimiterService,
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
    pub otp_persistence: Arc<dyn OtpPersistence>,
    pub login_rate_limiter: Arc<LoginRateLimiterService>,
}

impl AuthUseCase {
    pub fn new(
        user_persistence: Arc<dyn UserPersistence>,
        jwt_persistence: Arc<dyn JwtPersistence>,
        jwt_service: Arc<JwtService>,
        otp_service: Arc<OtpService>,
        otp_persistence: Arc<dyn OtpPersistence>,
        login_rate_limiter: Arc<LoginRateLimiterService>,
    ) -> Self {
        Self {
            user_persistence,
            jwt_persistence,
            jwt_service,
            otp_service,
            otp_persistence,
            login_rate_limiter,
        }
    }

    #[instrument(
        name= "use_case.auth.register_user",
        skip(self, session, username, email),
        fields(email=%email, username=%username)
    )]
    pub async fn register_user(
        &self,
        username: &str,
        email: &str,
        session: Session,
    ) -> AppResult<SuccessResponse<NewUserRequest>> {
        if let Some(user_exists) = self.user_persistence.find_by_email(email).await? {
            return self.handle_existing_user(user_exists, session).await;
        }

        let user_id = self.user_persistence.insert(username, email).await?;

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

    #[instrument(
        name = "use_case.auth.login_user",
        skip(self, auth_verifier),
        fields(email = %email)
    )]
    pub async fn login_user(
        &self,
        email: &str,
        auth_verifier: &str,
    ) -> AppResult<(PublicUser, AuthTokens)> {
        if let Some(retry_after) = self.login_rate_limiter.check_if_locked(email).await? {
            return Err(AppError::TooManyRequests {
                message: format!("Try again in {} seconds", retry_after),
                retry_after: Some(retry_after as u64),
            });
        }

        let (stored_verifier, username) =
            match self.user_persistence.find_verifier_by_email(email).await? {
                Some(verifier) => {
                    let username = self
                        .user_persistence
                        .find_by_email(email)
                        .await?
                        .map(|u| u.username)
                        .unwrap_or_default();
                    (verifier, username)
                }
                None => {
                    return Err(AppError::Unauthorized(
                        "Wrong email or password".to_string(),
                    ));
                }
            };

        // Constant-time comparison to prevent timing attacks
        if !constant_time_eq(auth_verifier.as_bytes(), stored_verifier.as_bytes()) {
            self.login_rate_limiter
                .record_failed_attempt(email, &username)
                .await?;

            return Err(AppError::Unauthorized(
                "Wrong email or password".to_string(),
            ));
        }

        self.login_rate_limiter.clear_attempts(email).await?;

        let user = self
            .user_persistence
            .find_public_user_by_email(email)
            .await?
            .ok_or(AppError::Unauthorized(
                "Wrong email or password".to_string(),
            ))?;

        // Create new token family for this login session
        let token_family = Uuid::new_v4();
        let access_token = self.jwt_service.create_access_token(user.id, email)?;
        let refresh_token = self
            .jwt_service
            .create_refresh_token(user.id, email, token_family)?;

        self.jwt_persistence
            .insert_rt(user.id, &refresh_token, token_family)
            .await?;

        Ok((
            user,
            AuthTokens {
                access_token,
                refresh_token,
            },
        ))
    }

    #[instrument(name = "use_case.user.handle_existing_user", skip(self, user, session))]
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
        name = "use_case.auth.verify_email_user",
        skip(self, session, otp_code, user_id)
    )]
    pub async fn verify_email_user(
        &self,
        user_id: Uuid,
        otp_code: &str,
        session: Session,
    ) -> AppResult<SuccessResponse<()>> {
        let otp_record = self
            .otp_persistence
            .find_by_id(user_id)
            .await?
            .ok_or(AppError::BadRequest("Invalid user id.".to_string()))?;

        if otp_record.code != otp_code {
            return Err(AppError::Unauthorized("Invalid OTP code.".to_string()));
        }

        if chrono::Utc::now() > otp_record.expires_at {
            return Err(AppError::Unauthorized("OTP code has expired.".to_string()));
        }

        self.user_persistence
            .update_email_verified_by_id(user_id)
            .await?;
        self.otp_persistence.delete_by_id(user_id).await?;

        remove_session(session.clone(), "verif_otp").await?;
        insert_session(session, "verif_password", user_id).await?;

        Ok(SuccessResponse {
            data: None,
            message: "Verified Otp success!".to_string(),
        })
    }

    #[instrument(
        name = "use_case.auth.update_user_identifier",
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
            .update_identifier_by_id(
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
            .find_by_id(user_id)
            .await?
            .ok_or(AppError::NotFound("User not found".to_string()))?;

        // Create new token family for this session
        let token_family = Uuid::new_v4();
        let access_token = self.jwt_service.create_access_token(user_id, &user.email)?;
        let refresh_token =
            self.jwt_service
                .create_refresh_token(user_id, &user.email, token_family)?;

        self.jwt_persistence
            .insert_rt(user_id, &refresh_token, token_family)
            .await?;

        Ok(AuthTokens {
            access_token,
            refresh_token,
        })
    }

    #[instrument(name = "use_case.auth.refresh_tokens_user", skip(self, refresh_token))]
    pub async fn refresh_tokens_user(&self, refresh_token: &str) -> AppResult<AuthTokens> {
        let claims = self.jwt_service.verify_refresh_token(refresh_token)?;
        let user_id = claims.sub;
        let token_family = claims.jti;

        let stored = self.jwt_persistence.find_rt_by_id(user_id).await?;

        match stored {
            Some(stored) if stored.is_revoked => {
                // Token family was revoked = reuse detected, force re-login
                self.jwt_persistence.delete_rt_by_id(user_id).await?;
                Err(AppError::Unauthorized(
                    "Session revoked. Please login again.".to_string(),
                ))
            }
            Some(stored) if stored.token == refresh_token => {
                // Valid token - rotate with same family
                let user = self
                    .user_persistence
                    .find_by_id(user_id)
                    .await?
                    .ok_or(AppError::NotFound("User not found".to_string()))?;

                let new_access_token =
                    self.jwt_service.create_access_token(user_id, &user.email)?;

                let new_refresh_token =
                    self.jwt_service
                        .create_refresh_token(user_id, &user.email, token_family)?;

                self.jwt_persistence
                    .insert_rt(user_id, &new_refresh_token, token_family)
                    .await?;

                Ok(AuthTokens {
                    access_token: new_access_token,
                    refresh_token: new_refresh_token,
                })
            }
            Some(_) => {
                // Token doesn't match = reuse attempt with old token
                // Revoke entire family to protect user
                self.jwt_persistence.revoke_token_family_by_id(user_id).await?;
                Err(AppError::Unauthorized(
                    "Token reuse detected. All sessions revoked.".to_string(),
                ))
            }
            None => Err(AppError::Unauthorized("Invalid refresh token".to_string())),
        }
    }

    #[instrument(name = "use_case.auth.logout_user", skip(self))]
    pub async fn logout_user(&self, user_id: Uuid) -> AppResult<SuccessResponse<()>> {
        self.jwt_persistence.delete_rt_by_id(user_id).await?;

        Ok(SuccessResponse {
            data: None,
            message: "Logged out successfully".to_string(),
        })
    }

    #[instrument(name = "use_case.auth.check_session_user", skip(self, session))]
    pub async fn check_session_user(&self, session: Session) -> AppResult<CheckSessionResponse> {
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

    #[instrument(name = "use_case.auth.is_user_locked", skip(self), fields(email = %email))]
    pub async fn is_user_locked(&self, email: &str) -> AppResult<Option<i64>> {
        self.login_rate_limiter.check_if_locked(email).await
    }

    #[instrument(name = "use_case.auth.unlock_user_account", skip(self, token))]
    pub async fn unlock_user_account(&self, token: String) -> AppResult<SuccessResponse<()>> {
        let email = self
            .login_rate_limiter
            .unlock_with_token(&token)
            .await
            .map_err(|_| AppError::InvalidToken)?;

        Ok(SuccessResponse {
            data: None,
            message: format!("Account {} unlocked successfully", email),
        })
    }

    #[instrument(name = "use_case.auth.report_failed_attempt", skip(self), fields(email = %email))]
    pub async fn report_failed_attempt(&self, email: &str) -> AppResult<()> {
        if let Some(retry_after) = self.login_rate_limiter.check_if_locked(email).await? {
            return Err(AppError::TooManyRequests {
                message: format!(
                    "Too many failed attempts. Try again in {} seconds, You can unlock your account by clicking on the link in the email we sent you.",
                    retry_after
                ),
                retry_after: Some(retry_after as u64),
            });
        }

        let username = self
            .user_persistence
            .find_by_email(email)
            .await?
            .map(|u| u.username)
            .unwrap_or_else(|| "User".to_string());

        let result = self
            .login_rate_limiter
            .record_failed_attempt(email, &username)
            .await?;

        match result {
            crate::service::rate_limiter::LoginRateLimitStatus::Locked { retry_after } => {
                Err(AppError::TooManyRequests {
                    message: format!(
                        "Too many failed attempts. Try again in {} seconds, You can unlock your account by clicking on the link in the email we sent you.",
                        retry_after
                    ),
                    retry_after: Some(retry_after as u64),
                })
            }
            crate::service::rate_limiter::LoginRateLimitStatus::Allowed { .. } => Err(
                AppError::Unauthorized("Wrong email or password".to_string()),
            ),
        }
    }

    #[instrument(name = "use_case.auth.get_public_user_by_id", skip(self), fields(user_id = %user_id))]
    pub async fn get_public_user_by_id(&self, user_id: Uuid) -> AppResult<PublicUser> {
        let user = self
            .user_persistence
            .find_public_user_by_id(user_id)
            .await?
            .ok_or(AppError::NotFound("User not found".to_string()))?;

        Ok(user)
    }
}
