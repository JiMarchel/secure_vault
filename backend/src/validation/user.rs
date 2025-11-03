use serde::Deserialize;
use unicode_segmentation::UnicodeSegmentation;

use crate::model::app_error::{AppError, AppResult};

pub struct Username(String);
pub struct Email(String);
pub struct NewUser {
    pub username: Username,
    pub email: Email,
}

#[derive(Deserialize)]
pub struct NewUserRequest {
    pub username: String,
    pub email: String,
}

impl TryFrom<NewUserRequest> for NewUser {
    type Error = AppError;

    fn try_from(req: NewUserRequest) -> AppResult<Self> {
        Ok(Self {
            username: Username::parse(req.username)?,
            email: Email::parse(req.email)?,
        })
    }
}

impl Username {
    pub fn parse(s: String) -> AppResult<Username> {
        let is_empty_or_whitespace = s.trim().is_empty();
        let is_too_long = s.graphemes(true).count() > 256;
        let forbidden = ['/', '(', ')', '"', '<', '>', '\\', '{', '}'];
        let contains_forbidden_character = s.chars().any(|g| forbidden.contains(&g));

        if is_empty_or_whitespace || is_too_long || contains_forbidden_character {
            Err(AppError::BadRequest(format!("{s} Invalid username.")))
        } else {
            Ok(Self(s))
        }
    }
}

impl AsRef<str> for Username {
    fn as_ref(&self) -> &str {
        &self.0
    }
}

impl Email {
    pub fn parse(s: String) -> AppResult<Email> {
        if validator::ValidateEmail::validate_email(&s) {
            Ok(Self(s))
        } else {
            Err(AppError::BadRequest(format!("{s}: is not a valid email.")))
        }
    }
}

impl AsRef<str> for Email {
    fn as_ref(&self) -> &str {
        &self.0
    }
}
