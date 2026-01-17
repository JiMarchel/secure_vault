use std::sync::Arc;

use axum::{Json, Router, extract::State, routing::{get, post}};
use tracing::instrument;

use crate::{
    application::user::UserUseCase, controller::app_state::AppState, model::{
        app_error::{AppError, AppResult},
        response::SuccessResponse,
        user::{User, UserIdentifier},
    }, service::rate_limiter::{RateLimit, RateLimiter}, validation::user::{Email, EmailString}
};

pub fn router() -> Router<AppState> {
    Router::new()
        .route("/by-email", get(get_user_by_email))
        .route("/identifier", post(get_user_identifier))
}

#[instrument(
    name = "get_user_by_email",
    skip(user_use_case, payload),
    fields(payload)
)]
pub async fn get_user_by_email(
    State(user_use_case): State<Arc<UserUseCase>>,
    Json(payload): Json<EmailString>,
) -> AppResult<Json<SuccessResponse<User>>> {
    let email: Email = payload.try_into()?;
    let user = user_use_case.get_user_by_email(email.as_ref()).await?;

    let res = SuccessResponse {
        data: Some(user),
        message: "User fetched successfully".to_string(),
    };
    Ok(Json(res))
}

#[instrument(
    name = "get_user_identifier",
    skip(user_use_case, rate_limiter, payload),
    fields(payload)
)]
pub async fn get_user_identifier(
    State(user_use_case): State<Arc<UserUseCase>>,
    State(rate_limiter): State<Arc<RateLimiter>>,
    Json(payload): Json<EmailString>,
) -> AppResult<Json<SuccessResponse<UserIdentifier>>> {
    let email: Email = payload.try_into()?;

    rate_limiter.increment_email_attempts(email.as_ref()).await?;

    let email_result = rate_limiter
        .check_email_limit(email.as_ref(), 10)
        .await?;

    match email_result {
        RateLimit::Locked { retry_after } => {
            return Err(AppError::TooManyRequests(format!(
                "Account locked, Try again in {} seconds",
                retry_after
            )));
        }
        RateLimit::Allowed { remaining } => {
            if remaining <= 5 {
                return Err(AppError::BadRequest(format!(
                    "You have {} remaining attempts left.",
                    remaining
                )));
            }
        }
    }

    let user = user_use_case.get_user_identifier(email.as_ref()).await?;

    let res = SuccessResponse {
        data: Some(user),
        message: "User fetched successfully".to_string(),
    };
    Ok(Json(res))
}
