use crate::model::app_error::{AppError, AppResult, ValidationErrorDetail};
use serde::{Deserialize, Serialize};
use unicode_segmentation::UnicodeSegmentation;

pub struct Username(String);
pub struct Email(String);

#[derive(Deserialize, Debug)]
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

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct LoginRequest {
    pub email: String,
    pub auth_verifier: String,
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
        let minimal_3_chars = s.graphemes(true).count() < 3;
        let is_too_long = s.graphemes(true).count() > 256;
        let forbidden = ['/', '(', ')', '"', '<', '>', '\\', '{', '}'];
        let contains_forbidden = s.chars().any(|g| forbidden.contains(&g));

        let message = if is_empty {
            "Username cannot be empty"
        } else if minimal_3_chars {
            "Username must be at least 3 characters"
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
        } else if !s.ends_with("@gmail.com") {
            Some(ValidationErrorDetail {
                field: "email".to_string(),
                message: "Email must be a gmail.com address".to_string(),
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_username_validation() {
        assert!(Username::to_validation_error("valid_user").is_none());

        let err = Username::to_validation_error("").unwrap();
        assert_eq!(err.field, "username");
        assert_eq!(err.message, "Username cannot be empty");

        let long_name = "a".repeat(257);
        let err = Username::to_validation_error(&long_name).unwrap();
        assert_eq!(err.field, "username");
        assert_eq!(err.message, "Username must be 256 characters or less");

        let err = Username::to_validation_error("user/name").unwrap();
        assert_eq!(err.field, "username");
        assert_eq!(err.message, "Username contains forbidden characters");
    }

    #[test]
    fn test_email_validation() {
        assert!(Email::to_validation_error("test@example.com").is_none());

        let err = Email::to_validation_error("invalid-email").unwrap();
        assert_eq!(err.field, "email");
        assert!(err.message.contains("not a valid email address"));
    }

    #[test]
    fn test_new_user_request_conversion() {
        let req = NewUserRequest {
            username: "valid_user".to_string(),
            email: "test@example.com".to_string(),
        };
        let result = NewUser::try_from(req);
        assert!(result.is_ok());

        let user = result.unwrap();
        assert_eq!(user.username.as_ref(), "valid_user");
        assert_eq!(user.email.as_ref(), "test@example.com");

        let req = NewUserRequest {
            username: "".to_string(),
            email: "invalid".to_string(),
        };
        let result = NewUser::try_from(req);
        assert!(result.is_err());

        match result {
            Err(AppError::ValidationError(errors)) => {
                assert_eq!(errors.len(), 2);
                assert!(errors.iter().any(|e| e.field == "username"));
                assert!(errors.iter().any(|e| e.field == "email"));
            }
            _ => panic!("Expected ValidationError"),
        }
    }
}
