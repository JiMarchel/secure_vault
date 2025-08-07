use std::sync::Arc;

use axum::{
    Router,
    routing::{get, patch, post},
    serve,
};
use sqlx::{Pool, Postgres};
use tokio::net::TcpListener;

use crate::routes::{
    get_user_basic_info::get_user_basic_info, get_verif_otp::get_verif_otp,
    health_check::health_check, sign_up::sign_up, update_verif_otp::update_verif_otp, verif_otp::verif_otp,
};

pub struct ApplicationState {
    pub pool: Pool<Postgres>,
}

pub async fn run(listener: TcpListener, pool: Pool<Postgres>) {
    let app_state = Arc::new(ApplicationState { pool });

    let app = Router::new()
        .route("/health-check", get(health_check))
        .route("/users/basic/{id}", get(get_user_basic_info))
        .route("/users/otp-code/{id}", get(get_verif_otp))
        .route("/users/otp-code/update/{id}", patch(update_verif_otp))
        .route("/users/otp-code/verif", patch(verif_otp))
        .route("/sign-up", post(sign_up))
        .with_state(app_state);

    serve(listener, app).await.unwrap();
}
