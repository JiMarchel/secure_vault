use crate::application::user::UserUseCase;
use crate::controller::app_state::AppState;
use crate::infra::config::AppConfig;
use crate::infra::db::init_db;
use crate::persistence::postgres::PostgresPersistence;
use crate::service::email::SmtpEmailService;
use crate::service::otp::RandomOtpGenerator;
use std::sync::Arc;

pub async fn init_app_state() -> anyhow::Result<AppState> {
    let config = AppConfig::from_env();
    
    //Database
    let pool = init_db().await?;
    let user_persistence = Arc::new(PostgresPersistence::new(pool.clone()));
    let otp_persistence = Arc::new(PostgresPersistence::new(pool.clone()));

    //services
    let otp_generator = Arc::new(RandomOtpGenerator);
    let email_service = Arc::new(SmtpEmailService::new(
        config.clone().smtp_host,
        config.clone().smtp_username,
        config.clone().smtp_password,
        config.clone().smtp_from_email,
    ));

    let user_use_case = UserUseCase::new(
        user_persistence,
        otp_persistence,
        otp_generator,
        email_service,
    );

    Ok(AppState {
        user_use_case: Arc::new(user_use_case),
    })
}
