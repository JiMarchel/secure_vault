//controller/users.rs
use std::sync::Arc;

use axum::{
    Json, Router,
    extract::State,
    routing::{get, patch, post},
};
use tower_sessions::Session;
use tracing::{info, instrument};

use crate::{
    application::user::UserUseCase,
    controller::app_state::AppState,
    model::{
        app_error::{AppError, AppResult}, jwt::AuthTokens, otp::{OtpRecord, VerifyOtpPayload}, response::SuccessResponse, user::{CheckSessionResponse, User, UserIndentifierPayload}
    },
    service::session::{get_any_session, get_session},
    validation::user::{ Email, EmailString, NewUser, NewUserRequest},
};

pub fn router() -> Router<AppState> {
    Router::new()
        .route("/sign-up", post(register))
        .route("/verify-otp", patch(verify_otp))
        .route("/update-user-identifier", patch(update_user_identifier))
        .route("/get-user-by-email", post(get_user_by_email))
        .route("/session/get-me", get(get_current_user_with_session))
        .route("/session/resend-otp", patch(send_otp_with_session))
        .route("/session/get-otp", get(get_otp_with_session))
        .route("/check-session", get(check_session))
}

#[instrument(
    name= "register_user", 
    skip(session, user_use_case, payload), 
    fields(email=%payload.email, username=%payload.username)
)]
pub async fn register(
    session: Session,
    State(user_use_case): State<Arc<UserUseCase>>,
    Json(payload): Json<NewUserRequest>,
) -> AppResult<Json<SuccessResponse<NewUserRequest>>> {
    info!("Processing user registration");

    let new_user: NewUser = payload.try_into()?;

    let res = user_use_case
        .sign_up(new_user.username.as_ref(), new_user.email.as_ref(), session)
        .await?;

    info!("User registration completed successfully");
    Ok(Json(res))
}

#[instrument(
    name= "verify_otp", 
    skip(session, user_use_case, payload), 
    fields(otp_length=%payload.otp_code.len())
)]
pub async fn verify_otp(
    session: Session,
    State(user_use_case): State<Arc<UserUseCase>>,
    Json(payload): Json<VerifyOtpPayload>,
) -> AppResult<Json<SuccessResponse<()>>> {
    info!("Processing OTP verification");
    let user_id = get_session(session.clone(), "verif_otp").await?;

    let res = user_use_case
        .verify_user_email(user_id, &payload.otp_code, session)
        .await?;

    info!("OTP verification completed successfully");
    Ok(Json(res))
}

#[instrument(
    name= "get_user_by_email", 
    skip(user_use_case, payload), 
    fields(payload)
)]
pub async fn get_user_by_email(
    State(user_use_case): State<Arc<UserUseCase>>,
    Json(payload): Json<EmailString>,
) -> AppResult<Json<SuccessResponse<User>>> {
    let email: Email = payload.try_into()?;
    let is_user_exists = user_use_case.user_persistence.get_user_by_email(email.as_ref()).await?;
    let user = is_user_exists.ok_or_else(|| AppError::NotFound(format!("User with email {} not found", email.as_ref())))?;

    let res = SuccessResponse {
        data: Some(user),
        message: "User fetched successfully".to_string(),
    };
    info!("User with email {} fetched successfully", email.as_ref());
    Ok(Json(res))
}

#[instrument(
    name= "update_user_identifier", 
    skip(session, user_use_case, payload), 
)]
pub async fn update_user_identifier(
    session: Session,
    State(user_use_case): State<Arc<UserUseCase>>,
    Json(payload): Json<UserIndentifierPayload>
) -> AppResult<Json<SuccessResponse<AuthTokens>>> {
    let user_id = get_session(session.clone(), "verif_password").await?;
    
    let res = user_use_case.update_user_identifier(payload.encrypted_dek, payload.nonce, payload.salt, payload.argon2_params, user_id, session).await?;

    Ok(Json(res))
}

#[instrument(
    name= "get_current_user_with_session", 
    skip(session, user_use_case), 
)]
pub async fn get_current_user_with_session(
    session: Session,
    State(user_use_case): State<Arc<UserUseCase>>,
) -> AppResult<Json<SuccessResponse<User>>> {
    info!("Fetching current user using session");
    let user_id = get_any_session(session, &["verif_otp", "verif_password"]).await?;

    let user = user_use_case
        .user_persistence
        .get_user_by_id(user_id)
        .await?
        .ok_or(AppError::NotFound("User not found".to_string()))?;

    let res = SuccessResponse {
        data: Some(user),
        message: "Current user fetched successfully".to_string(),
    };

    info!("Current user fetched successfully");
    Ok(Json(res))
}

#[instrument(
    name= "send_otp_with_session", 
    skip(session, user_use_case), 
)]
pub async fn send_otp_with_session(
    session: Session,
    State(user_use_case): State<Arc<UserUseCase>>,
) -> AppResult<Json<SuccessResponse<()>>> {
    info!("Resending verification OTP");
    let user_id = get_session(session, "verif_otp").await?;

    let user = user_use_case
        .user_persistence
        .get_user_by_id(user_id)
        .await?
        .ok_or(AppError::Unauthorized("User not found".to_string()))?;

    let res = user_use_case
        .resend_verification_otp(user_id, &user.email, &user.username)
        .await?;

    info!("Verification OTP resent successfully");
    Ok(Json(res))
}

#[instrument(
    name= "get_otp_with_session", 
    skip(session, user_use_case), 
)]
pub async fn get_otp_with_session(
    session: Session,
    State(user_use_case): State<Arc<UserUseCase>>,
) -> AppResult<Json<SuccessResponse<OtpRecord>>> {
    info!("Fetching OTP using session");
    let user_id = get_session(session, "verif_otp").await?;

    let otp_record = user_use_case
        .otp_persistence
        .get_otp_by_user_id(user_id)
        .await?;

    let res = SuccessResponse {
        data: Some(otp_record),
        message: "OTP fetched successfully".to_string(),
    };

    info!("OTP fetched successfully");
    Ok(Json(res))
}

#[instrument(
    name= "check_session", 
    skip(session), 
)]
pub async fn check_session(session: Session) -> AppResult<Json<SuccessResponse<CheckSessionResponse>>> {
    info!("Checking session");
    if let Some(_) = get_session(session.clone(), "verif_otp").await.ok() {
        info!("Session found: verif_otp");
        return Ok(
            Json(SuccessResponse {
                data: Some(CheckSessionResponse {
                    authenticated: false,
                }),
                message: "verif_otp".to_string(),
            }),
        );
    }

    if let Some(_) = get_session(session, "verif_password").await.ok() {
        info!("Session found: verif_password");
        return Ok(
            Json(SuccessResponse {
                data: Some(CheckSessionResponse {
                    authenticated: false,
                }),
                message: "verif_password".to_string(),
            }),
        );
    }

    //TODO Check if user is logged in

    info!("No relevant session found");
    Ok(
         Json(SuccessResponse {
              data: Some(CheckSessionResponse {
                authenticated: false,
            }),
            message: "No relevant session found".to_string(),
        }),
    )
}
