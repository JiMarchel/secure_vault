use axum::{Router, routing::get};

use crate::controller::{app_state::AppState, health_check::health_check};

pub mod app_state;
pub mod health_check;
pub mod user;

pub fn router() -> Router<AppState> {
    Router::new()
        .nest("/user", user::router())
        .route("/health", get(health_check))
}
