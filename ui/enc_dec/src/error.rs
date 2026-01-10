use thiserror::Error;

#[derive(Error, Debug)]
pub enum VaultError {
    #[error("Argon2 error: {0}")]
    Argon2(String),
    #[error("Wrong email or password")]
    DecryptionFailed,
    #[error("Encryption failed")]
    EncryptionFailed,
    #[error("Base64 decoding error: {0}")]
    Base64Error(#[from] base64::DecodeError),
    #[error("JSON error: {0}")]
    JsonError(#[from] serde_json::Error),
    #[error("RNG error: {0}")]
    RngError(String),
    #[error("Invalid parameters: {0}")]
    InvalidParams(String),
}

pub type AppResult<T> = std::result::Result<T, VaultError>;

impl From<argon2::Error> for VaultError {
    fn from(err: argon2::Error) -> Self {
        VaultError::Argon2(err.to_string())
    }
}
