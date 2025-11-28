use crate::model::app_error::{AppError, AppResult, ValidationErrorDetail};
use serde::{Deserialize, Serialize};
use unicode_segmentation::UnicodeSegmentation;

pub struct Username(String);
pub struct Email(String);

#[derive(Deserialize)]
pub struct EmailString {
    pub email: String,
}
pub struct NewUser {
    pub username: Username,
    pub email: Email,
}

#[derive(Deserialize, Serialize)]
pub struct NewUserRequest {
    pub username: String,
    pub email: String,
}

impl TryFrom<NewUserRequest> for NewUser {
    type Error = AppError;

    fn try_from(req: NewUserRequest) -> AppResult<Self> {
        let mut errors: Vec<ValidationErrorDetail> = Vec::new();

        if let Some(err) = Username::to_validation_error(&req.username) {
            errors.push(err);
        }

        if let Some(err) = Email::to_validation_error(&req.email) {
            errors.push(err);
        }

        if !errors.is_empty() {
            return Err(AppError::ValidationError(errors));
        }

        Ok(Self {
            username: Username(req.username),
            email: Email(req.email),
        })
    }
}

impl TryFrom<EmailString> for Email {
    type Error = AppError;

    fn try_from(value: EmailString) -> AppResult<Self> {
        let mut errors: Vec<ValidationErrorDetail> = Vec::new();

        if let Some(err) = Email::to_validation_error(&value.email) {
            errors.push(err);
        }

        if !errors.is_empty() {
            return Err(AppError::ValidationError(errors));
        }

        Ok(Email(value.email))
    }
}

impl Username {
    pub fn to_validation_error(s: &str) -> Option<ValidationErrorDetail> {
        let is_empty = s.trim().is_empty();
        let is_too_long = s.graphemes(true).count() > 256;
        let forbidden = ['/', '(', ')', '"', '<', '>', '\\', '{', '}'];
        let contains_forbidden = s.chars().any(|g| forbidden.contains(&g));

        let message = if is_empty {
            "Username cannot be empty"
        } else if is_too_long {
            "Username must be 256 characters or less"
        } else if contains_forbidden {
            "Username contains forbidden characters"
        } else {
            return None;
        };

        Some(ValidationErrorDetail {
            field: "username".to_string(),
            message: message.to_string(),
        })
    }
}

impl AsRef<str> for Username {
    fn as_ref(&self) -> &str {
        &self.0
    }
}

impl Email {
    pub fn to_validation_error(s: &str) -> Option<ValidationErrorDetail> {
        if !validator::ValidateEmail::validate_email(&s) {
            Some(ValidationErrorDetail {
                field: "email".to_string(),
                message: format!("'{}' is not a valid email address", s),
            })
        } else {
            None
        }
    }
}

impl AsRef<str> for Email {
    fn as_ref(&self) -> &str {
        &self.0
    }
}
