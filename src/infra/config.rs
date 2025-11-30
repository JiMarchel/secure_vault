use std::env;

use secrecy::{ExposeSecret, SecretBox};
use sqlx::postgres::PgConnectOptions;

use crate::model::app_error::{AppError, AppResult};

pub struct AppConfig {
    pub smtp: SmtpConfig,
    pub jwt: JwtConfig,
    pub database: DatabaseConfig,
    pub rust_log: String,
    pub environment: Environment,
}

pub struct SmtpConfig {
    pub host: String,
    pub username: String,
    pub password: SecretBox<String>,
    pub from_email: String,
}

pub struct JwtConfig {
    pub secret: SecretBox<String>,
}

pub struct DatabaseConfig {
    pub database_name: String,
    pub username: String,
    pub password: SecretBox<String>,
    pub host: String,
    pub port: u16,
}

#[derive(PartialEq)]
pub enum Environment {
    Development,
    Production,
}

impl AppConfig {
    pub fn from_env() -> Self {
        let environment = match get_env("ENVIRONMENT").unwrap_or_else(|_| "development".to_string()).as_str() {
            "production" => Environment::Production,
            _ => Environment::Development,
        };

        Self {
            smtp: SmtpConfig {
                host: get_env("SMTP_HOST").expect("SMTP_HOST not found"),
                username: get_env("SMTP_USERNAME").expect("SMTP_USERNAME not found"),
                password: SecretBox::new(Box::new(get_env("SMTP_PASSWORD").expect("SMTP_PASSWORD not found"))),
                from_email: get_env("SMTP_FROM_EMAIL").expect("SMTP_FROM_EMAIL not found"),
            },
            jwt: JwtConfig {
                secret: SecretBox::new(Box::new(get_env("JWT_SECRET").expect("JWT_SECRET not found"))),
            },
            database: DatabaseConfig {
                username: get_env("POSTGRES_USER").expect("POSTGRES_USER not found"),
                database_name: get_env("POSTGRES_DB_NAME").expect("POSTGRES_DB_NAME not found"),
                password: SecretBox::new(Box::new(get_env("POSTGRES_PASSWORD").expect("POSTGRES_PASSWORD not found"))),
                host: get_env("DB_HOST").expect("DB_HOST not found"),
                port: get_env("DB_PORT").expect("DB_PORT not found").parse().unwrap(),
            },
            rust_log: get_env("RUST_LOG").expect("RUST_LOG not found"),
            environment,
        }
    }

    pub fn is_production(&self) -> bool {
        self.environment == Environment::Production
    }
}

fn get_env(key: &str) -> AppResult<String> {
    env::var(key).map_err(|e| AppError::Internal(e.to_string()))
}

impl DatabaseConfig {
    /// When you not specify database name, it will use username as the name of the database
    pub fn without_db(&self) -> PgConnectOptions {
        PgConnectOptions::new()
            .host(&self.host)
            .username(&self.username)
            .password(&self.password.expose_secret())
            .port(self.port)
    }

    pub fn with_db(&self) -> PgConnectOptions {
        self.without_db().database(&self.database_name)
    }
}
