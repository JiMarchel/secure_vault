use thiserror::Error;

#[derive(Debug, Error)]
pub enum AppError {
    #[error("Database error: {0}")]
    Database(String),

    #[error("Invalid credentials")]
    InvalidCredentials,

    #[error("Internal error: {0}")]
    Internal(String),

    #[error("Data Not Found")]
    NotFound,

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
}

pub type AppResult<T> = Result<T, AppError>;
