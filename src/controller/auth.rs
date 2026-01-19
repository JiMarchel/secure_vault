use std::sync::Arc;

use axum::{
    Json, Router,
    extract::State,
    routing::{delete, get, patch, post},
};
use tower_sessions::Session;
use tracing::instrument;

use crate::{
    application::auth::AuthUseCase,
    controller::app_state::AppState,
    model::{
        app_error::{AppError, AppResult},
        jwt::{AuthTokens, Claims},
        otp::VerifyOtpPayload,
        response::SuccessResponse,
        user::{UnlockAccount, UserIdentifier, UserInfo},
    },
    service::session::{destroy_session, get_session},
    validation::user::{Email, EmailString, LoginRequest, NewUser, NewUserRequest},
};
use axum_extra::extract::cookie::{Cookie, CookieJar, SameSite};
use time::Duration;

pub fn router() -> Router<AppState> {
    Router::new()
        .route("/", post(register))
        .route("/me", get(get_current_user))
        .route("/verif/otp", patch(verify_otp))
        .route("/verif/identifier", patch(update_user_identifier))
        .route("/refresh", post(refresh))
        .route("/logout", delete(logout))
        .route("/login", post(login))
        .route("/report-failed", post(report_failed_attempt))
        .route("/unlock-account", post(unlock_account))
}

#[instrument(
    name= "register",
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
    name="login",
    skip(auth_use_case, jar),
    fields(email=%payload.email)
)]
pub async fn login(
    jar: CookieJar,
    State(auth_use_case): State<Arc<AuthUseCase>>,
    Json(payload): Json<LoginRequest>,
) -> AppResult<(CookieJar, Json<SuccessResponse<UserInfo>>)> {
    let user_email: Email = EmailString {
        email: payload.email,
    }
    .try_into()?;

    let res = auth_use_case
        .login(user_email.as_ref(), &payload.auth_verifier)
        .await?;

    let access_cookie = Cookie::build(("sv_at", res.1.access_token))
        .max_age(Duration::minutes(15))
        .http_only(true)
        .secure(false)
        .same_site(SameSite::Lax)
        .path("/")
        .build();

    let refresh_cookie = Cookie::build(("sv_rt", res.1.refresh_token))
        .max_age(Duration::days(7))
        .http_only(true)
        .secure(false)
        .same_site(SameSite::Lax)
        .path("/")
        .build();

    let jar = jar.add(access_cookie).add(refresh_cookie);

    Ok((
        jar,
        Json(SuccessResponse {
            data: Some(res.0),
            message: "Login success".to_string(),
        }),
    ))
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

#[instrument(name = "logout_user", skip(jar, auth_use_case, claims))]
pub async fn logout(
    jar: CookieJar,
    claims: Claims,
    State(auth_use_case): State<Arc<AuthUseCase>>,
) -> AppResult<(CookieJar, Json<SuccessResponse<()>>)> {
    let user_id = claims.sub;

    let res = auth_use_case.logout_user(user_id).await?;

    let jar = jar
        .remove(Cookie::build("sv_at").path("/").build())
        .remove(Cookie::build("sv_rt").path("/").build());

    Ok((jar, Json(res)))
}

#[instrument(name = "update_user_identifier", skip(session, auth_use_case, payload))]
pub async fn update_user_identifier(
    session: Session,
    jar: CookieJar,
    State(auth_use_case): State<Arc<AuthUseCase>>,
    Json(payload): Json<UserIdentifier>,
) -> AppResult<(CookieJar, Json<SuccessResponse<AuthTokens>>)> {
    let user_id = get_session(session.clone(), "verif_password").await?;

    let tokens = auth_use_case
        .update_user_identifier(
            payload.encrypted_dek,
            payload.nonce,
            payload.salt,
            payload.argon2_params,
            payload.auth_verifier,
            user_id,
            session.clone(),
        )
        .await?;

    destroy_session(session).await?;

    let access_cookie = Cookie::build(("sv_at", tokens.access_token))
        .max_age(Duration::minutes(15))
        .http_only(true)
        .secure(false)
        .same_site(SameSite::Lax)
        .path("/")
        .build();

    let refresh_cookie = Cookie::build(("sv_rt", tokens.refresh_token))
        .max_age(Duration::days(7))
        .http_only(true)
        .secure(false)
        .same_site(SameSite::Lax)
        .path("/")
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

#[instrument(name = "refresh_token", skip(auth_use_case, jar))]
pub async fn refresh(
    jar: CookieJar,
    State(auth_use_case): State<Arc<AuthUseCase>>,
) -> AppResult<(CookieJar, Json<SuccessResponse<AuthTokens>>)> {
    let refresh_token = jar
        .get("sv_rt")
        .map(|cookie| cookie.value().to_owned())
        .ok_or(AppError::Unauthorized("Missing refresh token".to_string()))?;

    let tokens = auth_use_case.refresh_tokens(&refresh_token).await?;

    let access_cookie = Cookie::build(("sv_at", tokens.access_token.clone()))
        .max_age(Duration::minutes(15))
        .http_only(true)
        .secure(false) // TODO: Set to true in production
        .same_site(SameSite::Lax)
        .path("/")
        .build();

    let refresh_cookie = Cookie::build(("sv_rt", tokens.refresh_token.clone()))
        .max_age(Duration::days(7))
        .http_only(true)
        .secure(false)
        .same_site(SameSite::Lax)
        .path("/")
        .build();

    let jar = jar.add(access_cookie).add(refresh_cookie);

    Ok((
        jar,
        Json(SuccessResponse {
            data: Some(tokens),
            message: "Token refreshed".to_string(),
        }),
    ))
}

/// Report a failed login attempt from client-side decryption failure.
/// This is called by the frontend when password decryption fails (Poly1305 auth tag mismatch).
#[instrument(
    name = "report_failed_attempt",
    skip(auth_use_case),
    fields(email = %payload.email)
)]
pub async fn report_failed_attempt(
    State(auth_use_case): State<Arc<AuthUseCase>>,
    Json(payload): Json<EmailString>,
) -> AppResult<Json<SuccessResponse<()>>> {
    let email: Email = payload.try_into()?;

    auth_use_case.report_failed_attempt(email.as_ref()).await?;

    Ok(Json(SuccessResponse {
        data: None,
        message: "".to_string(),
    }))
}

#[instrument(name = "unlock_account", skip(auth_use_case, payload))]
pub async fn unlock_account(
    State(auth_use_case): State<Arc<AuthUseCase>>,
    Json(payload): Json<UnlockAccount>,
) -> AppResult<Json<SuccessResponse<()>>> {
    let res = auth_use_case
        .unlock_account_with_token(payload.token)
        .await?;

    Ok(Json(res))
}

#[instrument(name = "get_current_user", skip(auth_use_case, claims))]
pub async fn get_current_user(
    claims: Claims,
    State(auth_use_case): State<Arc<AuthUseCase>>,
) -> AppResult<Json<SuccessResponse<UserInfo>>> {
    let user = auth_use_case.get_user_info_by_id(claims.sub).await?;

    Ok(Json(SuccessResponse {
        data: Some(user),
        message: "User retrieved successfully".to_string(),
    }))
}
