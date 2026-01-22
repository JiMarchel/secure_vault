use axum::{Router, routing::get};

use crate::controller::{app_state::AppState, health_check::health_check};

pub mod app_state;
pub mod auth;
pub mod health_check;
pub mod session;
pub mod user;
pub mod vault;

pub fn router() -> Router<AppState> {
    Router::new()
        .nest("/user", user::router())
        .nest("/auth", auth::router())
        .nest("/session", session::router())
        .route("/health", get(health_check))
}
