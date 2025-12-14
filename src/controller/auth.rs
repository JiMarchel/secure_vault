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
        app_error::AppResult, jwt::AuthTokens, otp::VerifyOtpPayload, response::SuccessResponse,
        user::UserIndentifierPayload,
    },
    service::session::get_session,
    validation::user::{NewUser, NewUserRequest},
};
use axum_extra::extract::cookie::{Cookie, CookieJar, SameSite};
use time::Duration;

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

pub async fn logout(
    session: Session,
    jar: CookieJar,
    State(auth_use_case): State<Arc<AuthUseCase>>,
) -> AppResult<(CookieJar, Json<SuccessResponse<()>>)> {
    let _ = auth_use_case.check_session_status(session); // Placeholder for actual logout logic if needed

    let jar = jar
        .remove(Cookie::from("access_token"))
        .remove(Cookie::from("refresh_token"));

    Ok((
        jar,
        Json(SuccessResponse {
            data: None,
            message: "Logged out successfully".to_string(),
        }),
    ))
}

#[instrument(name = "update_user_identifier", skip(session, auth_use_case, payload))]
pub async fn update_user_identifier(
    session: Session,
    jar: CookieJar,
    State(auth_use_case): State<Arc<AuthUseCase>>,
    Json(payload): Json<UserIndentifierPayload>,
) -> AppResult<(CookieJar, Json<SuccessResponse<AuthTokens>>)> {
    let user_id = get_session(session.clone(), "verif_password").await?;

    let tokens = auth_use_case
        .update_user_identifier(
            payload.encrypted_dek,
            payload.nonce,
            payload.salt,
            payload.argon2_params,
            user_id,
            session,
        )
        .await?;

    let access_cookie = Cookie::build(("access_token", tokens.access_token))
        .max_age(Duration::minutes(15))
        .http_only(true)
        .secure(false)
        .same_site(SameSite::Lax)
        .path("/")
        .build();

    let refresh_cookie = Cookie::build(("refresh_token", tokens.refresh_token))
        .max_age(Duration::days(7))
        .http_only(true)
        .secure(false)
        .same_site(SameSite::Lax)
        .path("/api/auth/refresh")
        .build();

    let jar = jar.add(access_cookie).add(refresh_cookie);

    Ok((
        jar,
        Json(SuccessResponse {
            data: None,
            message: "User identifier updated".to_string(),
        }),
    ))
}
