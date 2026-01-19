use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Claims {
    pub sub: Uuid,
    pub exp: i64,
    pub iat: i64,
    pub email: String,
}

#[derive(Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct RefreshClaims {
    pub sub: Uuid, // user_id
    pub exp: i64,
    pub iat: i64,
    pub email: String,
    pub jti: Uuid, // token family ID for rotation tracking
}

#[derive(Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct AuthTokens {
    pub access_token: String,
    pub refresh_token: String,
}

#[derive(Debug, Clone)]
pub struct StoredRefreshToken {
    pub token: String,
    pub token_family: Uuid,
    pub is_revoked: bool,
}
