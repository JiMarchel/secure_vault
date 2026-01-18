use std::sync::Arc;

use axum::extract::FromRef;

use crate::application::{auth::AuthUseCase, otp::OtpUseCase, user::UserUseCase};

#[derive(Clone)]
pub struct AppState {
    pub user_use_case: Arc<UserUseCase>,
    pub auth_use_case: Arc<AuthUseCase>,
    pub otp_use_case: Arc<OtpUseCase>,
}

impl FromRef<AppState> for Arc<UserUseCase> {
    fn from_ref(app_state: &AppState) -> Self {
        app_state.user_use_case.clone()
    }
}

impl FromRef<AppState> for Arc<AuthUseCase> {
    fn from_ref(app_state: &AppState) -> Self {
        app_state.auth_use_case.clone()
    }
}

impl FromRef<AppState> for Arc<OtpUseCase> {
    fn from_ref(app_state: &AppState) -> Self {
        app_state.otp_use_case.clone()
    }
}
