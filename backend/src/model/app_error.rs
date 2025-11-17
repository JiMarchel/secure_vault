use serde::Serialize;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum AppError {
    #[error("Database error: {0}")]
    Database(String),

    #[error("Invalid credentials")]
    InvalidCredentials,

    #[error("Internal error: {0}")]
    Internal(String),

    #[error("Data Not Found: {0}")]
    NotFound(String),

    #[error("Bad Request: {0}")]
    BadRequest(String),

    #[error("Unauthorized: {0}")]
    Unauthorized(String),

    #[error("Conflict{0}")]
    Conflict(String),

    #[error("Forbidden: {0}")]
    Forbidden(String),

    // JWT related errors
    #[error("JWT token creation failed: {0}")]
    TokenCreation(String),

    #[error("JWT token validation failed: {0}")]
    TokenValidation(String),

    #[error("Invalid JWT token")]
    InvalidToken,

    #[error("JWT token has expired")]
    ExpiredToken,

    #[error("Validation error")]
    ValidationError(Vec<ValidationErrorDetail>),
}

#[derive(Serialize, Debug)]
pub struct ValidationErrorDetail {
    pub field: String,
    pub message: String,
}

impl AppError {
    /// Get the error code for API responses
    pub fn error_code(&self) -> &'static str {
        match self {
            AppError::Database(_) => "DATABASE_ERROR",
            AppError::InvalidCredentials => "INVALID_CREDENTIALS",
            AppError::Internal(_) => "INTERNAL_ERROR",
            AppError::NotFound(_) => "NOT_FOUND",
            AppError::BadRequest(_) => "BAD_REQUEST",
            AppError::Unauthorized(_) => "UNAUTHORIZED",
            AppError::Conflict(_) => "CONFLICT",
            AppError::Forbidden(_) => "FORBIDDEN",
            AppError::TokenCreation(_) => "TOKEN_CREATION_FAILED",
            AppError::TokenValidation(_) => "TOKEN_VALIDATION_FAILED",
            AppError::InvalidToken => "INVALID_TOKEN",
            AppError::ExpiredToken => "EXPIRED_TOKEN",
            AppError::ValidationError(_) => "VALIDATION_ERROR",
        }
    }

    /// Get user-friendly message (hide internal details)
    pub fn user_message(&self) -> String {
        match self {
            AppError::Database(_) => "A database error occurred".to_string(),
            AppError::Internal(_) => "An internal server error occurred".to_string(),
            AppError::InvalidCredentials => "Invalid credentials provided".to_string(),
            AppError::NotFound(msg) => msg.clone(),
            AppError::BadRequest(msg) => msg.clone(),
            AppError::Unauthorized(msg) => msg.clone(),
            AppError::Conflict(msg) => msg.clone(),
            AppError::Forbidden(msg) => msg.clone(),
            AppError::TokenCreation(_) => "Failed to create authentication token".to_string(),
            AppError::TokenValidation(_) => "Invalid authentication token".to_string(),
            AppError::InvalidToken => "Invalid authentication token".to_string(),
            AppError::ExpiredToken => "Authentication token has expired".to_string(),
            AppError::ValidationError(_) => "Validation failed".to_string(),
        }
    }
}

pub type AppResult<T> = Result<T, AppError>;
