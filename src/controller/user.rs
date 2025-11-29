use std::sync::Arc;

use axum::{Json, Router, extract::State, routing::post};
use tracing::{instrument};

use crate::{
    application::user::UserUseCase,
    controller::app_state::AppState,
    model::{
        app_error::{AppResult},
        response::SuccessResponse,
        user::User,
    },
    validation::user::{Email, EmailString},
};

pub fn router() -> Router<AppState> {
    Router::new().route("/get-user-by-email", post(get_user_by_email))
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
    let user = user_use_case
        .get_user_by_email(email.as_ref())
        .await?;

    let res = SuccessResponse {
        data: Some(user),
        message: "User fetched successfully".to_string(),
    };
    Ok(Json(res))
}
