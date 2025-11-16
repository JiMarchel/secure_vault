use axum::{extract::Request, http::HeaderValue, middleware::Next, response::Response};
use uuid::Uuid;

pub const HEADER_X_REQUEST_ID: &str = "x-request-id";

#[derive(Clone)]
pub struct RequestId(pub String);

pub async fn request_id_middleware(mut request: Request, next: Next) -> Response {
    let request_id = extract_or_generate_request_id(&request);

    request
        .extensions_mut()
        .insert(RequestId(request_id.clone()));

    let mut response = next.run(request).await;

    if let Ok(header_value) = HeaderValue::from_str(&request_id) {
        response
            .headers_mut()
            .insert(HEADER_X_REQUEST_ID, header_value);
    }

    response
}

fn extract_or_generate_request_id(request: &Request) -> String {
    if let Some(existing_id) = request.headers().get(HEADER_X_REQUEST_ID) {
        if let Ok(id_str) = existing_id.to_str() {
            if let Ok(uuid) = Uuid::try_parse(id_str) {
                if uuid.get_version() == Some(uuid::Version::Random) {
                    return uuid.to_string();
                }
            }
        }
    }

    Uuid::new_v4().to_string()
}
