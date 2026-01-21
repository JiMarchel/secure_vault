use std::sync::Arc;
use tracing::instrument;
use uuid::Uuid;

use crate::{
    model::{
        app_error::{AppError, AppResult},
        user::{User, UserIdentifier},
    },
    service::{rate_limiter::{LoginRateLimiterService}, user::UserPersistence},
};

pub struct UserUseCase {
    pub user_persistence: Arc<dyn UserPersistence>,
    pub login_rate_limiter: Arc<LoginRateLimiterService>,
}

impl UserUseCase {
    pub fn new(
        user_persistence: Arc<dyn UserPersistence>,
        login_rate_limiter: Arc<LoginRateLimiterService>,
    ) -> Self {
        Self {
            user_persistence,
            login_rate_limiter,
        }
    }

    #[instrument(name = "use_case.get_user_by_email", skip(self, email))]
    pub async fn get_user_by_email(&self, email: &str) -> AppResult<User> {
        self.user_persistence
            .get_user_by_email(email)
            .await?
            .ok_or_else(|| AppError::NotFound(format!("User with email {} not found", email)))
    }

    #[instrument(name = "use_case.get_user_by_id", skip(self, user_id))]
    pub async fn get_user_by_id(&self, user_id: Uuid) -> AppResult<User> {
        self.user_persistence
            .get_user_by_id(user_id)
            .await?
            .ok_or_else(|| AppError::NotFound("User not found".into()))
    }

    pub async fn get_user_identifier(&self, email: &str) -> AppResult<UserIdentifier> {
        self.user_persistence
            .get_user_identifier(email)
            .await?
            .ok_or(AppError::Unauthorized(
                "Wrong email or password".to_string(),
            ))
    }

    #[instrument(name = "use_case.is_locked", skip(self), fields(email = %email))]
    pub async fn is_locked(&self, email: &str) -> AppResult<Option<i64>> {
        self.login_rate_limiter.check_if_locked(email).await
    }
}
