use std::sync::Arc;

use axum::extract::FromRef;

use crate::application::user::UserUseCase;

#[derive(Clone)]
pub struct AppState {
    pub user_use_case: Arc<UserUseCase>,
}

impl FromRef<AppState> for Arc<UserUseCase> {
    fn from_ref(app_state: &AppState) -> Self {
        app_state.user_use_case.clone()
    }
}
