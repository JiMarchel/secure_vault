use async_trait::async_trait;
use lettre::{
    Message, SmtpTransport, Transport,
    message::{Mailbox, header::ContentType},
    transport::smtp::authentication::Credentials,
};
use tracing::{error, info, instrument};

use crate::model::app_error::{AppError, AppResult};

#[async_trait]
pub trait EmailService: Send + Sync {
    async fn send_otp_email(&self, email: &str, username: &str, otp_code: &str) -> AppResult<()>;
    async fn send_email_async(&self, email: &str, username: &str, otp_code: &str) -> AppResult<()>;
}

#[derive(Clone)]
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
    #[instrument(
        name = "send_otp_email",
        skip(self),
        fields(email = %email, username = %username)
    )]
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

    async fn send_email_async(&self, email: &str, username: &str, otp_code: &str) -> AppResult<()> {
        let email = email.to_string();
        let username = username.to_string();
        let otp_code = otp_code.to_string();
        let self_clone = self.clone();

        tokio::spawn(async move {
            match self_clone
                .send_otp_email(&email, &username, &otp_code)
                .await
            {
                Ok(_) => {
                    info!("Email sent successfully to {}", email);
                }
                Err(e) => {
                    error!("Failed to send email to {}: {}", email, e);
                }
            }
        });
        Ok(())
    }
}
