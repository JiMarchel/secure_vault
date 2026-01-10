use tower_sessions::Session;
use uuid::Uuid;

use crate::model::app_error::{AppError, AppResult};

pub async fn get_session(session: Session, key: &str) -> AppResult<Uuid> {
    session
        .get::<Uuid>(key)
        .await?
        .ok_or_else(|| AppError::Unauthorized("Session not found or unauthorized".into()))
}

pub async fn insert_session(session: Session, key: &str, user_id: Uuid) -> AppResult<()> {
    session.insert(key, user_id).await?;

    Ok(())
}

pub async fn remove_session(session: Session, key: &str) -> AppResult<Option<Uuid>> {
    let uuid = session.remove(key).await?;

    Ok(uuid)
}

pub async fn destroy_session(session: Session) -> AppResult<()> {
    session.flush().await?;
    Ok(())
}

pub async fn get_any_session(session: Session, keys: &[&str]) -> AppResult<Uuid> {
    for key in keys {
        if let Ok(Some(value)) = session.get::<Uuid>(key).await {
            return Ok(value);
        }
    }
    Err(AppError::Unauthorized(
        "Session not found or unauthorized".into(),
    ))
}
