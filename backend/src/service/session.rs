use tower_sessions::Session;
use uuid::Uuid;

use crate::model::app_error::{AppError, AppResult};

pub async fn get_session(session: Session, key: &str) -> AppResult<Uuid> {
    match session.get::<Uuid>(key).await {
        Ok(Some(value)) => Ok(value),
        _ => Err(AppError::Unauthorized(
            "Session not found or unauthorized".to_string(),
        )),
    }
}

pub async fn insert_session(session: Session, key: &str, user_id: Uuid) -> AppResult<()> {
    session
        .insert(key, user_id)
        .await
        .map_err(|e| AppError::Internal(e.to_string()))
}

pub async fn remove_session(session: Session, key: &str) -> AppResult<Option<Uuid>> {
    session
        .remove(key)
        .await
        .map_err(|e| AppError::Internal(e.to_string()))
}

pub async fn get_any_session(session: Session, keys: &[&str]) -> AppResult<Uuid> {
    for key in keys {
        if let Ok(Some(value)) = session.get::<Uuid>(key).await {
            return Ok(value);
        }
    }
    Err(AppError::Unauthorized(
        "Session not found or unauthorized".to_string(),
    ))
}
