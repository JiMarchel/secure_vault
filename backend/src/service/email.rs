use async_trait::async_trait;
use lettre::{Message, SmtpTransport, Transport, message::{Mailbox, header::ContentType}, transport::smtp::authentication::Credentials};

use crate::model::app_error::{AppError, AppResult};

#[async_trait]
pub trait EmailService: Send + Sync {
    async fn send_otp_email(&self, email: &str, username: &str, otp_code: &str) -> AppResult<()>;
}

pub struct SmtpEmailService {
    smtp_host: String,
    smtp_username: String,
    smtp_password: String,
    from_email: String,
}

impl SmtpEmailService {
    pub fn new(
        smtp_host: String,
        smtp_username: String,
        smtp_password: String,
        from_email: String,
    ) -> Self {
        Self {
            smtp_host,
            smtp_username,
            smtp_password,
            from_email,
        }
    }
}

#[async_trait]
impl EmailService for SmtpEmailService {
    async fn send_otp_email(&self, email: &str, username: &str, otp_code: &str) -> AppResult<()> {
        let message = Message::builder()
            .from(Mailbox::new(
                Some("No Reply".to_owned()),
                self.from_email.parse().unwrap(),
            ))
            .to(Mailbox::new(
                Some(username.to_string()),
                email.parse().unwrap(),
            ))
            .subject("OTP CODE")
            .header(ContentType::TEXT_PLAIN)
            .body(otp_code.to_string())
            .unwrap();

        let creds = Credentials::new(self.smtp_username.clone(), self.smtp_password.clone());

        let transporter = SmtpTransport::relay(&self.smtp_host)
            .unwrap()
            .credentials(creds)
            .build();

        transporter
            .send(&message)
            .map_err(|e| AppError::Internal(format!("Failed to send email {e}")))?;

        Ok(())
    }
}
