use serde::Serialize;
use sqlx::{Pool, Postgres};
use uuid::Uuid;

use crate::{error::AppError};

#[derive(Serialize)]
pub struct UserBasicInfo {
    pub id: Uuid,
    pub username: String,
    pub email: String,
}

pub async fn fetch_user_basic_info(
    pool: &Pool<Postgres>,
    id: &str,
) -> Result<Option<UserBasicInfo>, AppError> {
    let valid_id = Uuid::parse_str(id).map_err(|_| AppError::BadRequest("Invalid Uuid format".into()))?;

    let maybe_user = sqlx::query_as!(
        UserBasicInfo,
        "SELECT id, username, email FROM users WHERE id = $1",
        valid_id
    )
    .fetch_optional(pool)
    .await?;

    if let Some(user) = maybe_user {
        Ok(Some(user))
    } else {
        Err(AppError::NotFound)
    }
}
