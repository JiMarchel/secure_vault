// src/infra/error.rs or wherever you have IntoResponse impl
use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
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

        let (status, error_response, retry_after) = match &self {
            AppError::Database(e) => {
                tracing::error!(database_error = e, "Database error occurred");
                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    ErrorResponse::new(user_message),
                    None,
                )
            }
            
            AppError::TooManyRequests { message, retry_after } => {
                let details = serde_json::json!({
                    "retry_after": retry_after,
                });
                (
                    StatusCode::TOO_MANY_REQUESTS,
                    ErrorResponse::with_details(message.clone(), details),
                    *retry_after,
                )
            }
            
            AppError::AccountLocked { retry_after } => {
                let details = serde_json::json!({
                    "retry_after": retry_after,
                    "locked": true,
                });
                (
                    StatusCode::FORBIDDEN,
                    ErrorResponse::with_details(user_message, details),
                    Some(*retry_after as u64),
                )
            }
            
            AppError::Redis(e) => {
                tracing::error!(redis_error = e, "Redis error occurred");
                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    ErrorResponse::new(user_message),
                    None,
                )
            }
            
            AppError::Internal(e) => {
                tracing::error!(internal_error = e, "Internal error occurred");
                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    ErrorResponse::new(user_message),
                    None,
                )
            }
            
            AppError::NotFound(_) => {
                (StatusCode::NOT_FOUND, ErrorResponse::new(user_message), None)
            }
            
            AppError::BadRequest(_) => {
                (StatusCode::BAD_REQUEST, ErrorResponse::new(user_message), None)
            }
            
            AppError::Unauthorized(_) => {
                (StatusCode::UNAUTHORIZED, ErrorResponse::new(user_message), None)
            }
            
            AppError::Conflict(_) => {
                (StatusCode::CONFLICT, ErrorResponse::new(user_message), None)
            }
            
            AppError::Forbidden(_) => {
                (StatusCode::FORBIDDEN, ErrorResponse::new(user_message), None)
            }
            
            AppError::InvalidCredentials => {
                (StatusCode::UNAUTHORIZED, ErrorResponse::new(user_message), None)
            }
            
            AppError::TokenCreation(e) => {
                tracing::error!(token_error = e, "Token creation failed");
                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    ErrorResponse::new(user_message),
                    None,
                )
            }
            
            AppError::TokenValidation(_) | AppError::InvalidToken | AppError::ExpiredToken => {
                (StatusCode::UNAUTHORIZED, ErrorResponse::new(user_message), None)
            }
            
            AppError::ValidationError(errors) => {
                let details = serde_json::json!({
                    "validation_errors": errors
                });
                (
                    StatusCode::BAD_REQUEST,
                    ErrorResponse::with_details(user_message, details),
                    None,
                )
            }
        };

        let mut response = (status, Json(error_response)).into_response();

        // Add Retry-After header for rate limiting
        if let Some(seconds) = retry_after {
            if let Ok(header_value) = seconds.to_string().parse() {
                response.headers_mut().insert("Retry-After", header_value);
            }
        }

        response
    }
}