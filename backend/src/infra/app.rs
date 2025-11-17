use axum::{
    Router,
    http::{
        HeaderValue, Method,
        header::{ACCEPT, AUTHORIZATION, CONTENT_TYPE, SET_COOKIE},
    },
    middleware::from_fn,
};
use tower_http::cors::CorsLayer;
use tower_sessions::SessionManagerLayer;
use tower_sessions_sqlx_store::PostgresStore;

use crate::{
    controller::{self, app_state::AppState},
    infra::{
        middleware::{
            error_handler::error_handler_middleware, request_id::request_id_middleware,
            tracing::create_trace_layer,
        },
        setup::init_tracing,
    },
};

pub fn create_app(app_state: AppState, session: SessionManagerLayer<PostgresStore>) -> Router {
    init_tracing();

    let cors = CorsLayer::new()
        .allow_origin("http://localhost:3000".parse::<HeaderValue>().unwrap()) // 👈 React dev server
        .allow_methods([
            Method::GET,
            Method::POST,
            Method::PUT,
            Method::DELETE,
            Method::OPTIONS,
        ])
        .allow_headers([CONTENT_TYPE, ACCEPT, AUTHORIZATION])
        .expose_headers([SET_COOKIE])
        .allow_credentials(true);

    Router::new()
        .nest("/api", controller::router())
        .with_state(app_state)
        .layer(create_trace_layer())
        .layer(from_fn(error_handler_middleware))
        .layer(from_fn(request_id_middleware))
        .layer(session)
        .layer(cors)
}
