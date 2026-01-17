use axum::{
    Json,
    http::StatusCode,
    response::{IntoResponse, Response},
};

use crate::model::{app_error::AppError, response::ErrorResponse};
impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        let error_code = self.error_code();
        let user_message = self.user_message();

        // Log the full error with details for debugging
        tracing::error!(
            error_code = error_code,
            error = ?self,
            "Request failed"
        );

        let (status, error_response) = match &self {
            AppError::Database(e) => {
                tracing::error!(database_error = e, "Database error occurred");
                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    ErrorResponse::new(user_message),
                )
            }
            AppError::TooManyRequests(_) => {
                (StatusCode::TOO_MANY_REQUESTS, ErrorResponse::new(user_message))
            }
            AppError::Redis(e) => {
                tracing::error!(redis_error = e, "Redis error occurred");
                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    ErrorResponse::new(user_message),
                )
            }
            AppError::Internal(e) => {
                tracing::error!(internal_error = e, "Internal error occurred");
                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    ErrorResponse::new(user_message),
                )
            }
            AppError::NotFound(_) => (StatusCode::NOT_FOUND, ErrorResponse::new(user_message)),
            AppError::BadRequest(_) => (StatusCode::BAD_REQUEST, ErrorResponse::new(user_message)),
            AppError::Unauthorized(_) => {
                (StatusCode::UNAUTHORIZED, ErrorResponse::new(user_message))
            }
            AppError::Conflict(_) => (StatusCode::CONFLICT, ErrorResponse::new(user_message)),
            AppError::Forbidden(_) => (StatusCode::FORBIDDEN, ErrorResponse::new(user_message)),
            AppError::InvalidCredentials => {
                (StatusCode::UNAUTHORIZED, ErrorResponse::new(user_message))
            }
            AppError::TokenCreation(e) => {
                tracing::error!(token_error = e, "Token creation failed");
                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    ErrorResponse::new(user_message),
                )
            }
            AppError::TokenValidation(_) | AppError::InvalidToken | AppError::ExpiredToken => {
                (StatusCode::UNAUTHORIZED, ErrorResponse::new(user_message))
            }
            AppError::ValidationError(errors) => {
                let details = serde_json::json!({
                    "validationErrors": errors
                });
                (
                    StatusCode::BAD_REQUEST,
                    ErrorResponse::with_details(user_message, details),
                )
            }
        };

        (status, Json(error_response)).into_response()
    }
}
