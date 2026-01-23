use redis::Client;
use secrecy::ExposeSecret;
use tower_sessions::cookie::SameSite;
use tower_sessions::cookie::time::Duration;
use tower_sessions::{Expiry, SessionManagerLayer};
use tower_sessions_sqlx_store::PostgresStore;

use crate::application::auth::AuthUseCase;
use crate::application::otp::OtpUseCase;
use crate::application::user::UserUseCase;
use crate::application::vault::VaultUseCase;
use crate::controller::app_state::AppState;
use crate::infra::config::AppConfig;
use crate::infra::db::get_connection_pool;
use crate::infra::telemetry::{get_subscriber, init_subscriber};
use crate::persistence::postgres::PostgresPersistence;
use crate::persistence::redis::RedisPersistence;
use crate::service::email::SmtpEmailService;
use crate::service::jwt::JwtService;
use crate::service::otp::OtpService;
use crate::service::rate_limiter::LoginRateLimiterService;
use std::sync::Arc;

pub struct AppDependencies {
    pub state: AppState,
    pub session_layer: SessionManagerLayer<PostgresStore>,
}

pub async fn init_app_state() -> anyhow::Result<AppDependencies> {
    let config = AppConfig::from_env();

    let pool = get_connection_pool(&config.database).await?;
    let pg_persistence = Arc::new(PostgresPersistence::new(pool.clone()));

    let redis_client = Client::open(config.redis_url.clone())?;
    let redis_conn = redis_client.get_connection_manager().await?;
    let redis_persistence = Arc::new(RedisPersistence::new(redis_conn));

    let session_store = PostgresStore::new(pool.clone());
    session_store.migrate().await?;

    let session_layer = SessionManagerLayer::new(session_store)
        .with_expiry(Expiry::OnInactivity(Duration::hours(24)))
        .with_secure(config.is_production())
        .with_name("auth_session")
        .with_http_only(true)
        .with_path("/")
        .with_same_site(SameSite::Lax);

    let email_service = Arc::new(SmtpEmailService::new(
        config.smtp.host.clone(),
        config.smtp.username.clone(),
        config.smtp.password.expose_secret().clone(),
        config.smtp.from_email.clone(),
    ));

    let jwt_service = Arc::new(JwtService::new(&config.jwt.secret.expose_secret().clone()));

    let otp_service = Arc::new(OtpService::new(
        redis_persistence.clone(),
        redis_persistence.clone(),
        email_service.clone(),
    ));

    let login_rate_limiter = Arc::new(LoginRateLimiterService::new(
        redis_persistence.clone(),
        redis_persistence.clone(),
        email_service.clone(),
    ));

    let user_use_case = Arc::new(UserUseCase::new(
        pg_persistence.clone(),
        login_rate_limiter.clone(),
    ));

    let auth_use_case = Arc::new(AuthUseCase::new(
        pg_persistence.clone(),
        pg_persistence.clone(),
        jwt_service.clone(),
        otp_service.clone(),
        redis_persistence.clone(),
        login_rate_limiter.clone(),
    ));

    let otp_use_case = Arc::new(OtpUseCase::new(
        redis_persistence.clone(),
        otp_service.clone(),
        redis_persistence.clone(),
        pg_persistence.clone(),
    ));

    let vault_use_case = Arc::new(VaultUseCase::new(
        pg_persistence.clone(),
    ));

    Ok(AppDependencies {
        state: AppState {
            user_use_case,
            auth_use_case,
            otp_use_case,
            vault_use_case,
        },
        session_layer,
    })
}

pub fn init_tracing() {
    let config = AppConfig::from_env();
    let subscriber = get_subscriber(config.rust_log, std::io::stdout);
    init_subscriber(subscriber);
}
