use serde::{Deserialize, Serialize};

#[derive(Serialize)]
pub struct CheckSessionResponse {
    pub authenticated: bool,
}

#[derive(Serialize, sqlx::FromRow)]
#[serde(rename_all = "camelCase")]
pub struct User {
    pub id: uuid::Uuid,
    pub username: String,
    pub email: String,
    pub encrypted_dek: Option<String>,
    pub salt: Option<String>,
    pub argon2_params: Option<String>,
    pub is_email_verified: bool,
    pub nonce: Option<String>,
    pub created_at: chrono::NaiveDateTime,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UserIndentifierPayload {
    pub encrypted_dek: String,
    pub salt: String,
    pub nonce: String,
    pub argon2_params: String,
}

impl User {
    pub fn is_pending_password_verification(&self) -> bool {
        self.is_email_verified && self.encrypted_dek.is_none()
    }

    pub fn is_pending_otp_verification(&self) -> bool {
        !self.is_email_verified
    }
}

#[cfg(test)]
mod tests {
    use chrono::Utc;
    use uuid::Uuid;

    use super::*;

    fn create_test_user() -> User {
        User {
            id: Uuid::new_v4(),
            username: "testuser".to_string(),
            email: "test@example.com".to_string(),
            is_email_verified: false,
            encrypted_dek: None,
            nonce: None,
            salt: None,
            argon2_params: None,
            created_at: Utc::now().naive_utc(),
        }
    }

    #[test]
    fn test_new_user_is_pending_otp() {
        let user = create_test_user();
        assert!(user.is_pending_otp_verification());
        assert!(!user.is_pending_password_verification());
    }

    #[test]
    fn test_email_verified_and_user_is_pending_password() {
        let mut user = create_test_user();
        user.is_email_verified = true;

        assert!(!user.is_pending_otp_verification());
        assert!(user.is_pending_password_verification());
    }

    #[test]
    fn test_fully_registered_user() {
        let mut user = create_test_user();
        user.is_email_verified = true;
        user.encrypted_dek = Some("encrypted".to_string());

        assert!(!user.is_pending_otp_verification());
        assert!(!user.is_pending_password_verification());
    }
}
