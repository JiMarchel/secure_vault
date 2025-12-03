use std::sync::Arc;

use axum::{
    Json, Router,
    extract::State,
    routing::{patch, post},
};
use tower_sessions::Session;
use tracing::instrument;

use crate::{
    application::auth::AuthUseCase,
    controller::app_state::AppState,
    model::{
        app_error::AppResult,
        jwt::AuthTokens,
        otp::VerifyOtpPayload,
        response::SuccessResponse,
        user::UserIndentifierPayload,
    },
    service::session::get_session,
    validation::user::{NewUser, NewUserRequest},
};

pub fn router() -> Router<AppState> {
    Router::new()
        .route("/", post(register))
        .route("/verif/otp", patch(verify_otp))
        .route("/verif/identifier", patch(update_user_identifier))
}

#[instrument(
    name= "register_user",
    skip(session, auth_use_case, payload),
    fields(email=%payload.email, username=%payload.username)
)]
pub async fn register(
    session: Session,
    State(auth_use_case): State<Arc<AuthUseCase>>,
    Json(payload): Json<NewUserRequest>,
) -> AppResult<Json<SuccessResponse<NewUserRequest>>> {
    let new_user: NewUser = payload.try_into()?;

    let res = auth_use_case
        .sign_up(new_user.username.as_ref(), new_user.email.as_ref(), session)
        .await?;

    Ok(Json(res))
}

#[instrument(
    name= "verify_otp",
    skip(session, auth_use_case, payload),
    fields(otp_length=%payload.otp_code.len())
)]
pub async fn verify_otp(
    session: Session,
    State(auth_use_case): State<Arc<AuthUseCase>>,
    Json(payload): Json<VerifyOtpPayload>,
) -> AppResult<Json<SuccessResponse<()>>> {
    let user_id = get_session(session.clone(), "verif_otp").await?;

    let res = auth_use_case
        .verify_user_email(user_id, &payload.otp_code, session)
        .await?;

    Ok(Json(res))
}

#[instrument(name = "update_user_identifier", skip(session, auth_use_case, payload))]
pub async fn update_user_identifier(
    session: Session,
    State(auth_use_case): State<Arc<AuthUseCase>>,
    Json(payload): Json<UserIndentifierPayload>,
) -> AppResult<Json<SuccessResponse<AuthTokens>>> {
    let user_id = get_session(session.clone(), "verif_password").await?;

    let res = auth_use_case
        .update_user_identifier(
            payload.encrypted_dek,
            payload.nonce,
            payload.salt,
            payload.argon2_params,
            user_id,
            session,
        )
        .await?;

    Ok(Json(res))
}


