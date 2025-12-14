use std::sync::Arc;

use axum::{
    extract::{FromRef, FromRequestParts},
    http::request::Parts,
};
use axum_extra::extract::CookieJar;

use crate::{
    application::auth::AuthUseCase,
    model::{app_error::AppError, jwt::Claims},
};

impl<S> FromRequestParts<S> for Claims
where
    S: Send + Sync,
    Arc<AuthUseCase>: FromRef<S>,
{
    type Rejection = AppError;

    async fn from_request_parts(parts: &mut Parts, state: &S) -> Result<Self, Self::Rejection> {
        let jar = CookieJar::from_headers(&parts.headers);

        let access_token = jar
            .get("access_token")
            .ok_or(AppError::Unauthorized("Missing access token".to_string()))?
            .value();

        let auth_use_case = Arc::<AuthUseCase>::from_ref(state);

        auth_use_case.jwt_service.verify_token(access_token)
    }
}
