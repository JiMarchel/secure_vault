use axum::{http::StatusCode, response::IntoResponse};
use thiserror::Error;
use validator::ValidationErrors;

#[derive(Debug, Error)]
pub enum AppError {
    #[error("Internal Server Error")]
    InternalServerError,
    #[error("Data Not Found")]
    NotFound,
    #[error("Bad Request: {0}")]
    BadRequest(String),
    #[error("Unauthorized")]
    Unauthorized,
    #[error("Validation Error {0}")]
    ValidationError(#[from] ValidationErrors),
    #[error(transparent)]
    SqlxError(#[from] sqlx::Error),
}

impl IntoResponse for AppError {
    fn into_response(self) -> axum::response::Response {
        let (status, error_message) = match self {
            AppError::InternalServerError => (
                StatusCode::INTERNAL_SERVER_ERROR,
                "Internal server error".to_string(),
            ),
            AppError::NotFound => (StatusCode::NOT_FOUND, "Data not found".to_string()),
            AppError::BadRequest(msg) => (StatusCode::BAD_REQUEST, msg),
            AppError::Unauthorized => (StatusCode::UNAUTHORIZED, "Unauthorized".to_string()),
            AppError::ValidationError(e) => {
                let message = format!("{e}").replace("\n", ", ");
                (StatusCode::BAD_REQUEST, message)
            }
            AppError::SqlxError(e) => {
                eprintln!("Database error: {e:?}");
                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    "Database error".to_string(),
                )
            }
        };

        let body = serde_json::json!({"error": error_message});
        (status, axum::Json(body)).into_response()
    }
}
