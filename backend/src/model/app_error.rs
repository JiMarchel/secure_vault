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
}

pub type AppResult<T> = Result<T, AppError>;
