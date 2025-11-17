use axum::{
    Extension, body::Body, extract::Request, http::Response as HttpResponse, middleware::Next,
    response::Response,
};
use http_body_util::BodyExt;

use crate::infra::middleware::request_id::RequestId;

pub async fn error_handler_middleware(
    Extension(request_id): Extension<RequestId>,
    request: Request,
    next: Next,
) -> Response {
    let response = next.run(request).await;

    if !response.status().is_server_error() {
        return response;
    }

    match inject_request_id_to_response(response, request_id.0).await {
        Ok(modified_response) => modified_response,
        Err(original_response) => {
            tracing::warn!("Failed to inject request ID into error response");
            original_response
        }
    }
}

async fn inject_request_id_to_response(
    response: Response,
    request_id: String,
) -> Result<Response, Response> {
    let (parts, body) = response.into_parts();

    let bytes = match body.collect().await {
        Ok(collected) => collected.to_bytes(),
        Err(_) => return Err(HttpResponse::from_parts(parts, Body::empty())),
    };

    let mut json_value: serde_json::Value = match serde_json::from_slice(&bytes) {
        Ok(json) => json,
        Err(_) => return Err(HttpResponse::from_parts(parts, Body::from(bytes))),
    };

    if json_value.get("error").is_some() {
        json_value["requestId"] = serde_json::json!(request_id);

        let modyfied_bytes = match serde_json::to_vec(&json_value) {
            Ok(b) => b,
            Err(_) => return Err(HttpResponse::from_parts(parts, Body::from(bytes))),
        };

        Ok(HttpResponse::from_parts(parts, Body::from(modyfied_bytes)))
    } else {
        Err(HttpResponse::from_parts(parts, Body::from(bytes)))
    }
}
