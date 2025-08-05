use std::sync::Arc;

use axum::{
    Json,
    extract::{Path, State},
};
use uuid::Uuid;

use crate::{
    error::AppError,
    helper::fetch_user_basic_info::{ UserBasicInfo, fetch_user_basic_info},
    startup::ApplicationState,
};

pub async fn get_user_basic_info(
    State(app_state): State<Arc<ApplicationState>>,
    Path(user_id): Path<Uuid>,
) -> Result<Json<UserBasicInfo>, AppError> {
    let maybe_user = fetch_user_basic_info(&app_state.pool, user_id).await?;

    if let Some(user) = maybe_user {
        Ok(Json(user))
    } else {
        Err(AppError::NotFound)
    }
}
