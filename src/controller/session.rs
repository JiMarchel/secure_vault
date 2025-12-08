use std::sync::Arc;

use axum::{
    Json, Router,
    extract::State,
    routing::{get, patch},
};
use tower_sessions::Session;
use tracing::instrument;

use crate::{
    application::{auth::AuthUseCase, otp::OtpUseCase, user::UserUseCase},
    controller::app_state::AppState,
    model::{
        app_error::AppResult,
        otp::{OtpExpiresAt, OtpRecord},
        response::SuccessResponse,
        user::{CheckSessionResponse, User},
    },
    service::session::{get_any_session, get_session},
};

pub fn router() -> Router<AppState> {
    Router::new()
        .route("/me", get(get_current_user_with_session))
        .route("/otp", get(get_otp_with_session))
        .route("/otp/expire", get(get_otp_expire_with_session))
        .route("/otp/resend", patch(send_otp_with_session))
        .route("/check", get(check_session))
}

#[instrument(name = "get_current_user_with_session", skip(session, user_use_case))]
pub async fn get_current_user_with_session(
    session: Session,
    State(user_use_case): State<Arc<UserUseCase>>,
) -> AppResult<Json<SuccessResponse<User>>> {
    let user_id = get_any_session(session, &["verif_otp", "verif_password"]).await?;

    let user = user_use_case.get_user_by_id(user_id).await?;

    let res = SuccessResponse {
        data: Some(user),
        message: "Current user fetched successfully".to_string(),
    };

    Ok(Json(res))
}

#[instrument(
    name = "send_otp_with_session",
    skip(session, otp_use_case, user_use_case)
)]
pub async fn send_otp_with_session(
    session: Session,
    State(otp_use_case): State<Arc<OtpUseCase>>,
    State(user_use_case): State<Arc<UserUseCase>>,
) -> AppResult<Json<SuccessResponse<()>>> {
    let user_id = get_session(session, "verif_otp").await?;

    let user = user_use_case.get_user_by_id(user_id).await?;

    let res = otp_use_case
        .resend_verification_otp(user_id, &user.email, &user.username)
        .await?;

    Ok(Json(res))
}

#[instrument(name = "get_otp_with_session", skip(session, otp_use_case))]
pub async fn get_otp_with_session(
    session: Session,
    State(otp_use_case): State<Arc<OtpUseCase>>,
) -> AppResult<Json<SuccessResponse<OtpRecord>>> {
    let user_id = get_session(session, "verif_otp").await?;

    let res = otp_use_case.get_otp_by_user_id(user_id).await?;

    Ok(Json(res))
}

#[instrument(name = "get_otp_expire_with_session", skip(session, otp_use_case))]
pub async fn get_otp_expire_with_session(
    session: Session,
    State(otp_use_case): State<Arc<OtpUseCase>>,
) -> AppResult<Json<SuccessResponse<OtpExpiresAt>>> {
    let user_id = get_session(session, "verif_otp").await?;

    let res = otp_use_case.get_otp_expire_by_user_id(user_id).await?;

    Ok(Json(res))
}

#[instrument(name = "check_session", skip(session, auth_use_case))]
pub async fn check_session(
    State(auth_use_case): State<Arc<AuthUseCase>>,
    session: Session,
) -> AppResult<Json<SuccessResponse<CheckSessionResponse>>> {
    let status = auth_use_case.check_session_status(session).await?;

    Ok(Json(SuccessResponse {
        data: Some(status),
        message: "Session checked".to_string(),
    }))
}
