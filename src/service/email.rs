use async_trait::async_trait;
use lettre::{
    Message, SmtpTransport, Transport,
    message::{Mailbox, header::ContentType},
    transport::smtp::authentication::Credentials,
};
use serde::Serialize;
use tracing::{error, instrument};

use crate::model::app_error::{AppError, AppResult};

#[async_trait]
pub trait EmailService: Send + Sync {
    async fn send(&self, payload: EmailPayload) -> AppResult<()>;
    async fn send_async(&self, payload: EmailPayload) -> AppResult<()>;
}

#[derive(Clone, Serialize, Debug)]
pub enum EmailTemplate {
    Otp {
        otp_code: String,
    },
    AccountLocked {
        unlock_token: String,
        expires_in: u64,
    },
}

#[derive(Clone, Debug)]
pub struct EmailPayload {
    pub to_email: String,
    pub to_username: String,
    pub template: EmailTemplate,
}

impl EmailPayload {
    pub fn subject(&self) -> &'static str {
        match &self.template {
            EmailTemplate::Otp { .. } => "Your OTP Code",
            EmailTemplate::AccountLocked { .. } => "Account Temporarily Locked",
        }
    }

    pub fn body(&self) -> String {
        match &self.template {
            EmailTemplate::Otp { otp_code } => {
                format!(
                    "Hello {}, \n\nYour OTP code is: {}\n\nThis code expires in 10 minutes.",
                    &self.to_username, otp_code
                )
            }
            EmailTemplate::AccountLocked {
                unlock_token,
                expires_in,
            } => {
                // TODO: Get base URL from config instead of hardcoding
                let unlock_url = format!(
                    "http://localhost:3000/verif/unlock-account/{}",
                    unlock_token
                );
                format!(
                    "Hello {},\n\nYour account is temporarily locked due to too many failed login attempts.\n\nYou can unlock it by clicking this link:\n{}\n\nIf this was not you, please ignore this email. Your account will automatically unlock in {} minutes.",
                    &self.to_username, unlock_url, expires_in
                )
            }
        }
    }
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
        name = "send_email_service",
        skip(self),
        fields(email = %payload.to_email)
    )]
    async fn send(&self, payload: EmailPayload) -> AppResult<()> {
        let message = Message::builder()
            .from(Mailbox::new(
                Some("No Reply".to_owned()),
                self.from_email.parse().unwrap(),
            ))
            .to(Mailbox::new(
                Some(payload.to_username.clone()),
                payload.to_email.parse().unwrap(),
            ))
            .subject(payload.subject())
            .header(ContentType::TEXT_PLAIN)
            .body(payload.body())
            .unwrap();
        let creds = Credentials::new(self.smtp_username.clone(), self.smtp_password.clone());
        let transporter = SmtpTransport::relay(&self.smtp_host)
            .unwrap()
            .credentials(creds)
            .build();
        transporter
            .send(&message)
            .map_err(|e| AppError::Internal(format!("Failed to send email: {e}")))?;
        Ok(())
    }

    #[instrument(
        name = "send_async_email_service",
        skip(self),
        fields(email = %payload.to_email)
    )]
    async fn send_async(&self, payload: EmailPayload) -> AppResult<()> {
        let self_clone = self.clone();
        tokio::spawn(async move {
            if let Err(e) = self_clone.send(payload).await {
                error!("Failed to send async email: {}", e);
            }
        });

        Ok(())
    }
}
