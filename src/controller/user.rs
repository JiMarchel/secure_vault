use std::sync::Arc;

use axum::{
    Json, Router,
    extract::State,
    routing::{get, post},
};
use tracing::instrument;

use crate::{
    application::user::UserUseCase,
    controller::app_state::AppState,
    model::{
        app_error::{AppError, AppResult},
        response::SuccessResponse,
        user::{User, UserIdentifier},
    },
    validation::user::{Email, EmailString},
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
    skip(user_use_case, payload),
    fields(payload)
)]
pub async fn get_user_identifier(
    State(user_use_case): State<Arc<UserUseCase>>,
    Json(payload): Json<EmailString>,
) -> AppResult<Json<SuccessResponse<UserIdentifier>>> {
    let email: Email = payload.try_into()?;

    if let Some(retry_after) = user_use_case.is_locked(email.as_ref()).await? {
        return Err(AppError::TooManyRequests {
            message: format!(
                "Too many failed attempts. Try again in {} seconds, You can unlock your account by clicking on the link in the email we sent you.",
                retry_after
            ),
            retry_after: Some(retry_after as u64),
        });
    }

    let user = user_use_case.get_user_identifier(email.as_ref()).await?;

    let res = SuccessResponse {
        data: Some(user),
        message: "User fetched successfully".to_string(),
    };
    Ok(Json(res))
}
