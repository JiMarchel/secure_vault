use async_trait::async_trait;
use chrono::{Duration, Utc};
use jsonwebtoken::{DecodingKey, EncodingKey, Header, decode, encode};
use uuid::Uuid;

use crate::model::{
    app_error::{AppError, AppResult},
    jwt::Claims,
};

#[async_trait]
pub trait JwtPersistence: Send + Sync {
    async fn create_refresh_token(&self, user_id: Uuid, email: &str) -> AppResult<()>;
    async fn delete_refresh_token(&self, user_id: Uuid) -> AppResult<()>;
}

pub struct JwtService {
    encoding_key: EncodingKey,
    decoding_key: DecodingKey,
    access_token_duration: Duration,
    refresh_token_duration: Duration,
}

impl JwtService {
    pub fn new(secret: &str) -> Self {
        Self {
            encoding_key: EncodingKey::from_secret(secret.as_bytes()),
            decoding_key: DecodingKey::from_secret(secret.as_bytes()),
            access_token_duration: Duration::minutes(10),
            refresh_token_duration: Duration::days(7),
        }
    }

    pub fn create_access_token(&self, user_id: Uuid, email: &str) -> AppResult<String> {
        let now = Utc::now();
        let exp = now + self.access_token_duration;

        let claims = Claims {
            sub: user_id,
            exp: exp.timestamp(),
            iat: now.timestamp(),
            email: email.to_string(),
        };

        encode(&Header::default(), &claims, &self.encoding_key)
            .map_err(|e| AppError::TokenCreation(e.to_string()))
    }

    pub fn create_refresh_token(&self, user_id: Uuid, email: &str) -> AppResult<String> {
        let now = Utc::now();
        let exp = now + self.refresh_token_duration;

        let claims = Claims {
            sub: user_id,
            exp: exp.timestamp(),
            iat: now.timestamp(),
            email: email.to_string(),
        };

        encode(&Header::default(), &claims, &self.encoding_key)
            .map_err(|e| AppError::TokenCreation(e.to_string()))
    }

    pub fn verify_token(&self, token: &str) -> AppResult<Claims> {
        let token_data = decode::<Claims>(
            token,
            &self.decoding_key,
            &jsonwebtoken::Validation::default(),
        )
        .map_err(|e| match e.kind() {
            jsonwebtoken::errors::ErrorKind::InvalidToken => AppError::InvalidToken,
            jsonwebtoken::errors::ErrorKind::ExpiredSignature => AppError::ExpiredToken,
            _ => AppError::TokenValidation(e.to_string()),
        })?;

        Ok(token_data.claims)
    }

    pub fn get_user_id_from_token(&self, token: &str) -> AppResult<Uuid> {
        let claims = self.verify_token(token)?;

        Ok(claims.sub)
    }
}
