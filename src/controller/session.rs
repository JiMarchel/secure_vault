use std::sync::Arc;

use axum::{
    Json, Router,
    extract::State,
    routing::{get, patch, post},
};
use tower_sessions::Session;
use tracing::instrument;

use crate::{
    application::{auth::AuthUseCase, otp::OtpUseCase, user::UserUseCase},
    controller::app_state::AppState,
    model::{
        app_error::AppResult,
        otp::{OtpStatusResponse, ResendOtpResponse, VerifyOtpRequest},
        response::SuccessResponse,
        user::{CheckSessionResponse, User},
    },
    service::session::{get_any_session, get_session, insert_session, remove_session},
};

pub fn router() -> Router<AppState> {
    Router::new()
        .route("/me", get(get_current_user_with_session))
        .route("/otp/status", get(get_otp_status_with_session))
        .route("/otp/resend", patch(resend_otp_with_session))
        .route("/otp/verify", post(verify_otp_with_session))
        .route("/check", get(check_session))
}

#[instrument(name = "get_current_user_with_session", skip(session, user_use_case))]
pub async fn get_current_user_with_session(
    session: Session,
    State(user_use_case): State<Arc<UserUseCase>>,
) -> AppResult<Json<SuccessResponse<User>>> {
    let user_id = get_any_session(session, &["verif_otp", "verif_password"]).await?;
    let user = user_use_case.get_user_by_id(user_id).await?;

    Ok(Json(SuccessResponse {
        data: Some(user),
        message: "Current user fetched successfully".to_string(),
    }))
}

#[instrument(name = "get_otp_status_with_session", skip(session, otp_use_case))]
pub async fn get_otp_status_with_session(
    session: Session,
    State(otp_use_case): State<Arc<OtpUseCase>>,
) -> AppResult<Json<SuccessResponse<OtpStatusResponse>>> {
    let user_id = get_session(session, "verif_otp").await?;
    let res = otp_use_case.get_otp_status(user_id).await?;
    Ok(Json(res))
}

#[instrument(
    name = "resend_otp_with_session",
    skip(session, otp_use_case, user_use_case)
)]
pub async fn resend_otp_with_session(
    session: Session,
    State(otp_use_case): State<Arc<OtpUseCase>>,
    State(user_use_case): State<Arc<UserUseCase>>,
) -> AppResult<Json<SuccessResponse<ResendOtpResponse>>> {
    let user_id = get_session(session, "verif_otp").await?;
    let user = user_use_case.get_user_by_id(user_id).await?;

    let res = otp_use_case
        .resend_otp_verification(user_id, &user.email, &user.username)
        .await?;

    Ok(Json(res))
}

#[instrument(name = "verify_otp_with_session", skip(session, otp_use_case, payload))]
pub async fn verify_otp_with_session(
    session: Session,
    State(otp_use_case): State<Arc<OtpUseCase>>,
    Json(payload): Json<VerifyOtpRequest>,
) -> AppResult<Json<SuccessResponse<()>>> {
    let user_id = get_session(session.clone(), "verif_otp").await?;
    let res = otp_use_case.verify_otp(user_id, &payload.otp_code).await?;

    remove_session(session.clone(), "verif_otp").await?;
    insert_session(session, "verif_password", user_id).await?;

    Ok(Json(res))
}

#[instrument(name = "check_session", skip(session, auth_use_case))]
pub async fn check_session(
    State(auth_use_case): State<Arc<AuthUseCase>>,
    session: Session,
) -> AppResult<Json<SuccessResponse<CheckSessionResponse>>> {
    let status = auth_use_case.check_session_user(session).await?;

    Ok(Json(SuccessResponse {
        data: Some(status),
        message: "Session checked".to_string(),
    }))
}
