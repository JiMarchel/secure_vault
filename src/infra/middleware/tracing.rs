use tower_http::{
    classify::{ServerErrorsAsFailures, SharedClassifier},
    trace::{MakeSpan, TraceLayer},
};

use crate::infra::middleware::request_id::RequestId;

#[derive(Clone)]
pub struct CustomMakeSpan(pub String);

impl<B> MakeSpan<B> for CustomMakeSpan {
    fn make_span(&mut self, request: &axum::http::Request<B>) -> tracing::Span {
        let request_id = request
            .extensions()
            .get::<RequestId>()
            .map(|id| id.0.clone())
            .unwrap_or_else(|| "unkown".to_string());

        tracing::info_span!(
            "http-request",
            method = %request.method(),
            uri = %request.uri(),
            version = ?request.version(),
            request_id = %request_id
        )
    }
}

pub fn create_trace_layer() -> TraceLayer<SharedClassifier<ServerErrorsAsFailures>, CustomMakeSpan>
{
    TraceLayer::new_for_http().make_span_with(CustomMakeSpan("http-request".to_string()))
}
