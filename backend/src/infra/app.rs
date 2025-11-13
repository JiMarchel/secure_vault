use axum::{
    Router,
    http::{
        self, HeaderValue, Method,
        header::{ACCEPT, AUTHORIZATION, CONTENT_TYPE, SET_COOKIE},
    },
};
use tower_http::{
    cors::CorsLayer,
    trace::{TraceLayer},
};
use tower_sessions::SessionManagerLayer;
use tower_sessions_sqlx_store::PostgresStore;
use uuid::Uuid;

use crate::{
    controller::{self, app_state::AppState},
    infra::setup::init_tracing,
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
        .layer(cors)
        .layer(session)
        .layer(
            TraceLayer::new_for_http().make_span_with(|req: &http::Request<_>| {
                let request_id = Uuid::new_v4();
                tracing::info_span!(
                    "http-request",
                    method = %req.method(),
                    uri = %req.uri(),
                    version = ?req.version(),
                    request_id = %request_id
                )
            }),
        )
}
