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
                host: get_env("SMTP_HOST").unwrap(),
                username: get_env("SMTP_USERNAME").unwrap(),
                password: SecretBox::new(Box::new(get_env("SMTP_PASSWORD").unwrap())),
                from_email: get_env("SMTP_FROM_EMAIL").unwrap(),
            },
            jwt: JwtConfig {
                secret: SecretBox::new(Box::new(get_env("JWT_SECRET").unwrap())),
            },
            database: DatabaseConfig {
                username: get_env("DATABASE_NAME").unwrap(),
                database_name: get_env("DATABASE_USER").unwrap(),
                password: SecretBox::new(Box::new(get_env("DATABASE_PASSWORD").unwrap())),
                host: get_env("DATABASE_HOST").unwrap(),
                port: get_env("DATABASE_PORT").unwrap().parse().unwrap(),
            },
            rust_log: get_env("RUST_LOG").unwrap(),
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
