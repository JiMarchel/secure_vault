use std::sync::{Arc, Mutex};

use async_trait::async_trait;
use backend::{
    application::{auth::AuthUseCase, otp::OtpUseCase, user::UserUseCase},
    controller::app_state::AppState,
    infra::{
        config::AppConfig,
        telemetry::{get_subscriber, init_subscriber},
    },
    model::app_error::{AppError, AppResult},
    persistence::postgres::PostgresPersistence,
    service::{email::EmailService, jwt::JwtService, otp::OtpService},
};
use once_cell::sync::Lazy;
use sqlx::{Connection, PgConnection, PgPool};
use wiremock::MockServer;

static TRACING: Lazy<()> = Lazy::new(|| {
    let config = AppConfig::from_env();

    if std::env::var("TEST_LOG").is_ok() {
        let subscriber = get_subscriber(config.rust_log, std::io::stdout);
        init_subscriber(subscriber);
    } else {
        let subscriber = get_subscriber(config.rust_log, std::io::sink);
        init_subscriber(subscriber);
    }
});

pub struct TestApp {
    pub address: String,
    pub pool: PgPool,
    pub email_service: Arc<FakeEmailService>
}

async fn configure_database(base_url: &str, db_name: &str) -> PgPool {
    let mut conn = PgConnection::connect_with(base_url).await.expect("Failed to connect postgres");

    conn.execute()
}

async fn build_test_app_state(pool: PgPool, email_service: Arc<FakeEmailService>) -> AppState {
    let persistence = Arc::new(PostgresPersistence::new(pool));


    let jwt_service = Arc::new(JwtService::new("test_secret"));

    let otp_service = Arc::new(OtpService::new(persistence.clone(), email_service.clone()));

    AppState {
        user_use_case: Arc::new(UserUseCase::new(persistence.clone())),
        auth_use_case: Arc::new(AuthUseCase::new(
            persistence.clone(),
            persistence.clone(),
            jwt_service.clone(),
            otp_service.clone(),
        )),
        otp_use_case: Arc::new(OtpUseCase::new(otp_service)),
    }
}

#[derive(Debug, Clone)]
pub struct CapturedEmail {
    pub to: String,
    pub username: String,
    pub otp_code: String,
}

/// Fake email service yang capture emails instead of sending
pub struct FakeEmailService {
    /// Store semua email yang "dikirim"
    pub sent_emails: Arc<Mutex<Vec<CapturedEmail>>>,
}

impl FakeEmailService {
    pub fn new() -> Self {
        Self {
            sent_emails: Arc::new(Mutex::new(Vec::new())),
        }
    }

    /// Get semua email yang sudah di-capture
    pub fn get_sent_emails(&self) -> Vec<CapturedEmail> {
        self.sent_emails.lock().unwrap().clone()
    }

    /// Get email terakhir yang dikirim
    pub fn get_last_email(&self) -> Option<CapturedEmail> {
        self.sent_emails.lock().unwrap().last().cloned()
    }

    /// Clear captured emails
    pub fn clear(&self) {
        self.sent_emails.lock().unwrap().clear();
    }

    /// Check apakah email terkirim ke address tertentu
    pub fn was_email_sent_to(&self, email: &str) -> bool {
        self.sent_emails
            .lock()
            .unwrap()
            .iter()
            .any(|e| e.to == email)
    }

    /// Get OTP yang dikirim ke email tertentu
    pub fn get_otp_for_email(&self, email: &str) -> Option<String> {
        self.sent_emails
            .lock()
            .unwrap()
            .iter()
            .find(|e| e.to == email)
            .map(|e| e.otp_code.clone())
    }
}

#[async_trait]
impl EmailService for FakeEmailService {
    async fn send_otp_email(&self, email: &str, username: &str, otp_code: &str) -> AppResult<()> {
        let captured = CapturedEmail {
            to: email.to_string(),
            username: username.to_string(),
            otp_code: otp_code.to_string(),
        };

        self.sent_emails.lock().unwrap().push(captured);

        tracing::info!(
            email = %email,
            username = %username,
            "Fake email captured (not actually sent)"
        );

        Ok(())
    }

    async fn send_email_async(&self, email: &str, username: &str, otp_code: &str) -> AppResult<()> {
        // Untuk test, langsung panggil synchronously supaya bisa di-assert
        self.send_otp_email(email, username, otp_code).await
    }
}