use crate::helpers::spawn_app;
use backend::model::{response::SuccessResponse, vault::Vaults};
use uuid::Uuid;

#[tokio::test]
async fn search_vault_works() {
    let app = spawn_app().await;

    // 1. Create User
    let user_id = Uuid::new_v4();
    let email = "test@example.com";

    // Using simple insert, assuming other fields are nullable or have defaults if not specified
    // Based on User struct, many are Option.
    // auth_verifier might be needed for login but here we generate token manually anyway.
    sqlx::query!(
        r#"
        INSERT INTO users (id, username, email, is_email_verified, created_at, auth_verifier)
        VALUES ($1, $2, $3, $4, NOW(), $5)
        "#,
        user_id,
        "testuser",
        email,
        true,
        "dummy_verifier"
    )
    .execute(&app.pool)
    .await
    .expect("Failed to insert user");

    // 2. Generate Token
    let token = app
        .jwt_service
        .create_access_token(user_id, email)
        .expect("Failed to create token");

    // 3. Create Vault Items
    let vault_id = Uuid::new_v4();
    let title = "My Secret Password";

    // vaults table likely expects item_type as string/varchar
    sqlx::query!(
        r#"
        INSERT INTO vaults (id, user_id, title, item_type, encrypted_data, nonce, created_at, updated_at)
        VALUES ($1, $2, $3, $4, $5, $6, NOW(), NOW())
        "#,
        vault_id,
        user_id,
        title,
        "Password",
        "encrypted_content",
        "nonce_val"
    )
    .execute(&app.pool)
    .await
    .expect("Failed to insert vault");

    // 4. Search Matching
    let client = reqwest::Client::new();
    let response = client
        .get(&format!("{}/vault/search?title=Secret", app.address))
        .header("Cookie", format!("sv_at={}", token))
        .send()
        .await
        .expect("Failed to execute request");

    assert_eq!(response.status().as_u16(), 200);

    let body = response
        .json::<SuccessResponse<Vec<Vaults>>>()
        .await
        .expect("Failed to parse json");
    let vaults = body.data.unwrap();

    assert_eq!(vaults.len(), 1);
    assert_eq!(vaults[0].title, title);

    // 5. Search Non-Matching
    let response = client
        .get(&format!("{}/vault/search?title=NotFound", app.address))
        .header("Cookie", format!("sv_at={}", token))
        .send()
        .await
        .expect("Failed to execute request");

    let body = response
        .json::<SuccessResponse<Vec<Vaults>>>()
        .await
        .expect("Failed to parse json");
    let vaults = body.data.unwrap();
    assert_eq!(vaults.len(), 0);
}
