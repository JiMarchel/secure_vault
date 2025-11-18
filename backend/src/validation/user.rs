use serde::{Deserialize, Serialize};
use unicode_segmentation::UnicodeSegmentation;
use crate::model::app_error::{AppError, AppResult, ValidationErrorDetail};

pub struct Username(String);
pub struct Email(String);

pub struct NewUser {
    pub username: Username,
    pub email: Email,
}

#[derive(Deserialize, Serialize)]
pub struct NewUserRequest {
    pub username: String,
    pub email: String,
}

struct NewUserValidator {
    errors: Vec<ValidationErrorDetail>,
}

impl NewUserValidator {
    fn new() -> Self {
        Self { errors: Vec::new() }
    }

    fn validate_username(&mut self, username: &str) -> Option<Username> {
        match Username::parse(username.to_string()) {
            Ok(u) => Some(u),
            Err(_) => {
                if let Some(error) = Username::to_validation_error(username) {
                    self.errors.push(error);
                }
                None
            }
        }
    }

    fn validate_email(&mut self, email: &str) -> Option<Email> {
        match Email::parse(email.to_string()) {
            Ok(e) => Some(e),
            Err(_) => {
                if let Some(error) = Email::to_validation_error(email) {
                    self.errors.push(error);
                }
                None
            }
        }
    }

    fn finish(self) -> Result<(), AppError> {
        if self.errors.is_empty() {
            Ok(())
        } else {
            Err(AppError::ValidationError(self.errors))
        }
    }
}

impl TryFrom<NewUserRequest> for NewUser {
    type Error = AppError;

    fn try_from(req: NewUserRequest) -> AppResult<Self> {
        let mut validator = NewUserValidator::new();

        let username = validator.validate_username(&req.username);
        let email = validator.validate_email(&req.email);

        validator.finish()?;

        Ok(Self {
            username: username.unwrap(),
            email: email.unwrap(),
        })
    }
}

impl Username {
    pub fn parse(s: String) -> AppResult<Username> {
        if let Some(error) = Self::to_validation_error(&s) {
            return Err(AppError::BadRequest(error.message));
        }
        Ok(Self(s))
    }

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
    pub fn parse(s: String) -> AppResult<Email> {
        if let Some(error) = Self::to_validation_error(&s) {
            return Err(AppError::BadRequest(error.message));
        }
        Ok(Self(s))
    }

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