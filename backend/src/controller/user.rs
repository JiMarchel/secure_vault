use std::sync::Arc;

use axum::{
    Json, Router,
    extract::State,
    http::StatusCode,
    response::IntoResponse,
    routing::{get, patch, post},
};
use tower_sessions::Session;

use crate::{
    application::user::UserUseCase,
    controller::app_state::AppState,
    model::{
        app_error::{AppError, AppResult}, otp::VerifyOtp, user::CheckSessionResponse
    },
    service::session::get_session,
    validation::user::{NewUser, NewUserRequest},
};

pub fn router() -> Router<AppState> {
    Router::new()
        .route("/sign-up", post(register))
        .route("/verify-otp", patch(verify_otp))
        .route("/session/get-me", get(get_current_user_with_session))
        .route("/session/resend-otp", patch(send_otp_with_session))
        .route("/session/get-otp", get(get_otp_with_session))
        .route("/check-session", get(check_session))
}

pub async fn register(
    session: Session,
    State(user_use_case): State<Arc<UserUseCase>>,
    Json(payload): Json<NewUserRequest>,
) -> AppResult<impl IntoResponse> {
    let new_user: NewUser = payload.try_into()?;

    let res = user_use_case
        .sign_up(new_user.username.as_ref(), new_user.email.as_ref(), session)
        .await?;

    Ok((StatusCode::CREATED, Json(res)))
}

pub async fn verify_otp(
    session: Session,
    State(user_use_case): State<Arc<UserUseCase>>,
    Json(payload): Json<VerifyOtp>,
) -> AppResult<impl IntoResponse> {
    let user_id = get_session(session.clone(), "verif_otp").await?;

    user_use_case.verify_user_email(user_id, &payload.otp_code, session).await?;
    Ok(StatusCode::OK)
}

pub async fn get_current_user_with_session(
    session: Session,
    State(user_use_case): State<Arc<UserUseCase>>,
) -> AppResult<impl IntoResponse> {
    let user_id = get_session(session, "verif_otp").await?;

    let user = user_use_case
        .user_persistence
        .get_user_by_id(user_id)
        .await?
        .ok_or(AppError::NotFound)?;

    Ok((StatusCode::OK, Json(user)))
}

pub async fn send_otp_with_session(
    session: Session,
    State(user_use_case): State<Arc<UserUseCase>>,
) -> AppResult<impl IntoResponse> {
    let user_id = get_session(session, "verif_otp").await?;

    let user = user_use_case
        .user_persistence
        .get_user_by_id(user_id)
        .await?
        .ok_or(AppError::Unauthorized("User not found".to_string()))?;

    user_use_case
        .resend_verification_otp(user_id, &user.email, &user.username)
        .await?;

    Ok(StatusCode::OK)
}

pub async fn get_otp_with_session(
    session: Session,
    State(user_use_case): State<Arc<UserUseCase>>,
) -> AppResult<impl IntoResponse> {
    let user_id = get_session(session, "verif_otp").await?;

    let otp_record = user_use_case
        .otp_persistence
        .get_otp_by_user_id(user_id)
        .await?;

    Ok((StatusCode::OK, Json(otp_record)))
}

pub async fn check_session(session: Session) -> AppResult<impl IntoResponse> {
    if let Some(_) = get_session(session.clone(), "verif_otp").await.ok() {
        return Ok((
            StatusCode::OK,
            Json(CheckSessionResponse {
                authenticated: false,
                message: Some("verif_otp".to_string()),
            }),
        ));
    }

    if let Some(_) = get_session(session, "verif_password").await.ok() {
        return Ok((
            StatusCode::OK,
            Json(CheckSessionResponse {
                authenticated: false,
                message: Some("verif_password".to_string()),
            }),
        ));
    }

    //TODO Check if user is logged in

    Ok((
        StatusCode::OK,
        Json(CheckSessionResponse {
            message: None,
            authenticated: false,
        }),
    ))
}
