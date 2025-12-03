use crate::helpers::spawn_app;

#[tokio::test]
async fn test_sign_up_sends_otp_email() {
    let app = spawn_app().await;

    let response = app.sign_up("example", "example@gmail.com").await;

    assert_eq!(response.status(), 200);

    assert!(app.email_service.was_email_sent_to("example@gmail.com"));
    let otp = app.email_service.get_otp_for_email("example@gmail.com");
    assert!(otp.is_some());
    assert_eq!(otp.unwrap().len(), 6);
}

#[tokio::test]
async fn test_verify_otp_with_captured_code() {
    let app = spawn_app().await;

    let response = app.sign_up("example", "example@gmail.com").await;
    assert_eq!(response.status(), 200);

    let cookies = response
        .headers()
        .get("set-cookie")
        .map(|v| v.to_str().unwrap())
        .unwrap();

    let otp = app
        .email_service
        .get_otp_for_email("example@gmail.com")
        .expect("OTP should have been captured");

    let response = app.verify_otp(&otp, cookies).await;

    assert_eq!(response.status(), 200);

    let user = app.get_user_by_email("example@gmail.com").await.unwrap();
    assert!(user.is_email_verified);
}
