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
    models::{RegisterPayload, SignUpResponse, User},
    startup::ApplicationState,
};

pub async fn sign_up(
    State(app_state): State<Arc<ApplicationState>>,
    Json(payload): Json<RegisterPayload>,
) -> Result<SignUpResponse, AppError> {
    payload.validate()?;

    let existing_user: Option<User> = sqlx::query_as(
        "SELECT id, username, email, encrypted_dek, salt, 
        argon2_params, is_email_verified, created_at FROM users WHERE email = $1",
    )
    .bind(&payload.email)
    .fetch_optional(&app_state.pool)
    .await?;

    if let Some(ref user) = existing_user {
        if user.is_email_verified && user.encrypted_dek.is_none() {
            return Ok(SignUpResponse::PendingVerification {
                message: "verif_password".to_string(),
                id: user.id,
            });
        } else if !user.is_email_verified {
            return Ok(SignUpResponse::PendingVerification {
                message: "verif_otp".to_string(),
                id: user.id,
            });
        } else {
            return Err(AppError::Conflict(
                "Email already taken, please login".to_string(),
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
    let expires_at = Utc::now() + Duration::minutes(10);

    let user_info: RegisterPayload = sqlx::query_as!(
        RegisterPayload,
        r#"
        WITH new_otp AS (
            INSERT INTO otp_verif (user_id, otp_code, otp_expires_at)
            VALUES ($1, $2, $3)
            RETURNING user_id
        )
        SELECT u.email as "email!", u.username as "username!"
        FROM users u
        JOIN new_otp n ON u.id = n.user_id
        "#,
        user_id,
        &otp_code,
        expires_at
    )
    .fetch_one(&app_state.pool)
    .await?;

    let email = Message::builder()
        .from(Mailbox::new(
            Some("No Reply".to_owned()),
            "no_reply@gmail.com".parse().unwrap(),
        ))
        .to(Mailbox::new(
            Some(user_info.username.clone()),
            user_info.email.parse().unwrap(),
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

    Ok(SignUpResponse::PendingVerification {
        message: "created".to_string(),
        id: user_id,
    })
}
