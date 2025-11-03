use std::sync::Arc;

use axum::{
    Json, Router, extract::State, http::StatusCode, response::IntoResponse, routing::get,
    routing::post,
};
use tower_sessions::Session;
use uuid::Uuid;

use crate::{
    application::user::UserUseCase,
    controller::app_state::AppState,
    model::{
        app_error::{AppError, AppResult},
        user::CheckSessionResponse,
    },
    validation::user::{NewUser, NewUserRequest},
};

pub fn router() -> Router<AppState> {
    Router::new()
        .route("/sign-up", post(register))
        .route("/get/me", get(get_current_user))
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

pub async fn get_current_user(
    session: Session,
    State(user_use_case): State<Arc<UserUseCase>>,
) -> AppResult<impl IntoResponse> {
    let user_id: Uuid = session
        .get("verif_otp")
        .await
        .map_err(|e| AppError::Internal(e.to_string()))?
        .ok_or(AppError::Unauthorized)?;

    let user = user_use_case
        .user_persistence
        .get_user_by_id(user_id)
        .await?
        .ok_or(AppError::NotFound)?;

    Ok((StatusCode::OK, Json(user)))
}

pub async fn check_session(session: Session) -> AppResult<impl IntoResponse> {
    if let Some(_) = session
        .get::<Uuid>("verif_otp")
        .await
        .map_err(|e| AppError::Internal(e.to_string()))?
    {
        return Ok((
            StatusCode::OK,
            Json(CheckSessionResponse {
                authenticated: false,
                message: Some("verif_otp".to_string()),
            }),
        ));
    }

    if let Some(_) = session
        .get::<Uuid>("verif_password")
        .await
        .map_err(|e| AppError::Internal(e.to_string()))?
    {
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
