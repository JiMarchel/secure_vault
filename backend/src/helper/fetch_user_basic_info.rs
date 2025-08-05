use sqlx::{ Pool, Postgres};
use uuid::Uuid;

pub struct UserBasicInfo {
    pub id: Uuid,
    pub username: String,
    pub email: String,
}

pub async fn fetch_user_basic_info(pool: &Pool<Postgres>, id: Uuid) -> Result<Option<UserBasicInfo>, sqlx::Error> {
    sqlx::query_as!(
        UserBasicInfo,
        "SELECT id, username, email FROM users WHERE id = $1",
        id
    )
    .fetch_optional(pool)
    .await
}
