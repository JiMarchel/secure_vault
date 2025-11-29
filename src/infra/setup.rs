use tracing_subscriber::layer::SubscriberExt;
use tracing_subscriber::util::SubscriberInitExt;
use tracing_subscriber::{EnvFilter, fmt};

use crate::application::auth::AuthUseCase;
use crate::application::otp::OtpUseCase;
use crate::application::user::UserUseCase;
use crate::controller::app_state::AppState;
use crate::infra::config::AppConfig;
use crate::infra::db::init_db;
use crate::persistence::postgres::PostgresPersistence;
use crate::service::email::SmtpEmailService;
use crate::service::jwt::JwtService;
use crate::service::otp::OtpService;
use std::sync::Arc;

pub async fn init_app_state() -> anyhow::Result<AppState> {
    let config = AppConfig::from_env();
    
    let pool = init_db().await?;
    let persistence = Arc::new(PostgresPersistence::new(pool));

    let email_service = Arc::new(SmtpEmailService::new(
        config.smtp_host.clone(),
        config.smtp_username.clone(),
        config.smtp_password.clone(),
        config.smtp_from_email.clone(),
    ));
    
    let jwt_service = Arc::new(JwtService::new(&config.jwt_secret.clone()));
    
    let otp_service = Arc::new(OtpService::new(
        persistence.clone(),
        email_service.clone(),
    ));

    let user_use_case = Arc::new(UserUseCase::new(
        persistence.clone(),
    ));
    
    let auth_use_case = Arc::new(AuthUseCase::new(
        persistence.clone(),
        persistence.clone(),
        jwt_service.clone(),
        otp_service.clone(),
    ));
    
    let otp_use_case = Arc::new(OtpUseCase::new(
        otp_service.clone(),
    ));

    Ok(AppState {
        user_use_case,
        auth_use_case,
        otp_use_case,
    })
}

pub fn init_tracing() {
    let filter = EnvFilter::try_from_default_env()
        .unwrap_or_else(|_| "info,backend=debug,tower_http=debug,sqlx=warn".into());

    let console_layer = fmt::layer().with_target(true).with_level(true).pretty();

    tracing_subscriber::registry()
        .with(filter)
        .with(console_layer)
        .init()
}
