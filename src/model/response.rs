use serde::Serialize;

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SuccessResponse<T> {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<T>,
    pub message: String,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ErrorResponse {
    pub error: ErrorDetail,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ErrorDetail {
    pub message: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub details: Option<serde_json::Value>,
}

impl<T: Serialize> SuccessResponse<T> {
    pub fn new(data: T, message: String) -> Self {
        Self {
            data: Some(data),
            message,
        }
    }
}

impl ErrorResponse {
    pub fn new(message: String) -> Self {
        Self {
            error: ErrorDetail {
                message,
                details: None,
            },
            request_id: None,
        }
    }

    pub fn with_details(message: String, details: serde_json::Value) -> Self {
        Self {
            error: ErrorDetail {
                message,
                details: Some(details),
            },
            request_id: None,
        }
    }
}
