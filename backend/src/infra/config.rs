use std::env;

#[derive(Clone)]
pub struct AppConfig {
    pub smtp_host: String,
    pub smtp_username: String,
    pub smtp_password: String,
    pub smtp_from_email: String,
}

impl AppConfig {
    pub fn from_env() -> Self {
        let smtp_host = env::var("SMTP_HOST").expect("SMTP_HOST");
        let smtp_username = env::var("SMTP_USERNAME").expect("SMTP_USERNAME");
        let smtp_password = env::var("SMTP_PASSWORD").expect("SMTP_PASSWORD");
        let smtp_from_email = env::var("SMTP_FROM_EMAIL").expect("SMTP_FROM_EMAIL");

        Self {
            smtp_host,
            smtp_username,
            smtp_password,
            smtp_from_email,
        }
    }
}
