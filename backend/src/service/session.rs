use tower_sessions::Session;
use uuid::Uuid;

use crate::model::app_error::{AppError, AppResult};

pub async fn get_session(session: Session, key: &str) -> AppResult<Uuid>  {
    match session.get::<Uuid>(key).await {
        Ok(Some(value)) => Ok(value),
        _ => Err(AppError::Unauthorized),
    }
}