use axum::{Json, extract::State};
use chrono::{Duration, Utc};
use lettre::{
    Message, SmtpTransport, Transport,
    message::{Mailbox, header::ContentType},
    transport::smtp::authentication::Credentials,
};
use rand::{Rng, rng};
use std::{env, sync::Arc};
use uuid::Uuid;
use validator::Validate;

use crate::{
    error::AppError,
    models::{AppResponse, RegisterPayload, User},
    startup::ApplicationState,
};

pub async fn sign_up(
    State(app_state): State<Arc<ApplicationState>>,
    Json(payload): Json<RegisterPayload>,
) -> Result<AppResponse, AppError> {
    payload.validate()?;

    let existing_user: Option<User> = sqlx::query_as(
        "SELECT id, username, email, encrypted_dek, salt, 
        argon2_params, is_email_verified, created_at FROM users WHERE email = $1",
    )
    .bind(&payload.email)
    .fetch_optional(&app_state.pool)
    .await?;

    if let Some(ref user) = existing_user {
        println!("{user:?}");
        if user.is_email_verified {
            return Ok(AppResponse::Message(
                "Email already taken, please login".to_string(),
            ));
        } else {
            return Ok(AppResponse::Redirect(
                "Email already registered but not verified".to_string(),
            ));
        }
    }

    let user_id: Uuid =
        sqlx::query_scalar("INSERT INTO users (username, email) VALUES($1, $2) RETURNING id")
            .bind(&payload.username)
            .bind(&payload.email)
            .fetch_one(&app_state.pool)
            .await?;

    let otp_code = format!("{:06}", rng().random_range(1..1_000_000));
    let expires_at = Utc::now() + Duration::minutes(5);

    sqlx::query("INSERT INTO otp_verif (user_id, otp_code, otp_expires_at) VALUES ($1, $2, $3)")
        .bind(user_id)
        .bind(&otp_code)
        .bind(expires_at)
        .execute(&app_state.pool)
        .await?;

    let email = Message::builder()
        .from(Mailbox::new(
            Some("No Reply".to_owned()),
            "jmarchel100@gmail.com".parse().unwrap(),
        ))
        .to(Mailbox::new(
            Some("Jimmy".to_owned()),
            "jmarchel200@gmail.com".parse().unwrap(),
        ))
        .subject("OTP CODE")
        .header(ContentType::TEXT_PLAIN)
        .body(otp_code.to_string())
        .unwrap();

    let creds = Credentials::new(
        "jmarchel100@gmail.com".to_string(),
        env::var("SMTP_PASSWORD").expect("SMTP password not set"),
    );

    let transporter = SmtpTransport::relay("smtp.gmail.com")
        .unwrap()
        .credentials(creds)
        .build();

    match transporter.send(&email) {
        Ok(_) => println!("Email sent successfully!"),
        Err(e) => panic!("Could not send email: {e:?}"),
    }

    Ok(AppResponse::Redirect(
        "Last step, please check your email".to_string(),
    ))
}
