use std::sync::{Arc, Mutex};

use async_trait::async_trait;
use backend::{
    application::{auth::AuthUseCase, otp::OtpUseCase, user::UserUseCase},
    controller::app_state::AppState,
    infra::{
        app::create_app,
        config::{AppConfig, DatabaseConfig},
        db::get_connection_pool,
        setup::AppDependencies,
        telemetry::{get_subscriber, init_subscriber},
    },
    model::{app_error::AppResult, user::User},
    persistence::postgres::PostgresPersistence,
    service::{email::EmailService, jwt::JwtService, otp::OtpService, rate_limiter::RateLimiter},
};
use once_cell::sync::Lazy;
use redis::Client;
use serde_json::json;
use sqlx::Executor;
use sqlx::{Connection, PgConnection, PgPool, postgres::PgPoolOptions};
use tokio::net::TcpListener;
use tower_sessions::{
    Expiry, SessionManagerLayer,
    cookie::{SameSite, time::Duration},
};
use tower_sessions_sqlx_store::PostgresStore;
use uuid::Uuid;

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
    pub email_service: Arc<FakeEmailService>,
}

impl TestApp {
    pub async fn sign_up(&self, username: &str, email: &str) -> reqwest::Response {
        reqwest::Client::new()
            .post(format!("{}/auth", &self.address))
            .json(&json!({
                "username": username,
                "email": email
            }))
            .send()
            .await
            .expect("Failed to send request")
    }

    pub async fn verify_otp(&self, otp_code: &str, cookies: &str) -> reqwest::Response {
        reqwest::Client::new()
            .patch(format!("{}/auth/verif/otp", self.address))
            .header("Cookie", cookies)
            .json(&json!({
                "otp_code": otp_code
            }))
            .send()
            .await
            .expect("Failed to send request")
    }

    pub async fn get_user_by_email(&self, email: &str) -> Option<User> {
        sqlx::query_as("SELECT * FROM users WHERE email = $1")
            .bind(email)
            .fetch_optional(&self.pool)
            .await
            .expect("Failed to query test")
    }
}

pub async fn spawn_app() -> TestApp {
    dotenvy::dotenv().ok();
    Lazy::force(&TRACING);

    let configuration = {
        let mut c = AppConfig::from_env();

        c.database.database_name = Uuid::new_v4().to_string();
        c
    };

    configure_database(&configuration.database).await;

    let pool = get_connection_pool(&configuration.database)
        .await
        .expect("Failed to connect to postgres");

    let email_service = Arc::new(FakeEmailService::new());

    let state = build_test_app_state(pool.clone(), email_service.clone(), configuration.redis_url)
        .await
        .expect("Failed to build app state");

    let app = create_app(state.state, state.session_layer);

    let listener = TcpListener::bind("localhost:0")
        .await
        .expect("Failed to bind port");
    let app_port = listener.local_addr().unwrap().port();
    let address = format!("http://localhost:{}/api", app_port);

    tokio::spawn(async move {
        axum::serve(listener, app).await.unwrap();
    });

    TestApp {
        address,
        pool,
        email_service,
    }
}

async fn configure_database(config: &DatabaseConfig) -> PgPool {
    let mut connection = PgConnection::connect_with(&config.without_db().database("postgres"))
        .await
        .expect("Failed to connect to postgres");

    connection
        .execute(format!(r#"CREATE DATABASE "{}";"#, config.database_name).as_str())
        .await
        .expect("Failed to create database");

    //migrate db
    let connection_pool = PgPoolOptions::new()
        .max_connections(10)
        .connect_with(config.with_db())
        .await
        .expect("Failed to connect postgres");

    sqlx::migrate!("./migrations")
        .run(&connection_pool)
        .await
        .expect("Failed to migrate database");

    connection_pool
}

async fn build_test_app_state(
    pool: PgPool,
    email_service: Arc<FakeEmailService>,
    redis_url: String,
) -> anyhow::Result<AppDependencies> {
    let persistence = Arc::new(PostgresPersistence::new(pool.clone()));

    let session_store = PostgresStore::new(pool.clone());
    session_store.migrate().await?;

    let session_layer = SessionManagerLayer::new(session_store)
        .with_expiry(Expiry::OnInactivity(Duration::hours(24)))
        .with_secure(false)
        .with_name("auth_session")
        .with_http_only(true)
        .with_path("/")
        .with_same_site(SameSite::Lax);

    let jwt_service = Arc::new(JwtService::new("test_secret"));

    let otp_service = Arc::new(OtpService::new(persistence.clone(), email_service.clone()));

    let redis_client = Client::open(redis_url).expect("Failed to connect redis");
    let redis_conn = redis_client
        .get_connection_manager()
        .await
        .expect("Failed to get Redis connection manager");

    let rate_limiter = Arc::new(RateLimiter::new(redis_conn));
    Ok(AppDependencies {
        state: AppState {
            user_use_case: Arc::new(UserUseCase::new(persistence.clone())),
            auth_use_case: Arc::new(AuthUseCase::new(
                persistence.clone(),
                persistence.clone(),
                jwt_service.clone(),
                otp_service.clone(),
            )),
            otp_use_case: Arc::new(OtpUseCase::new(otp_service.clone())),
            rate_limiter,
        },
        session_layer,
    })
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
