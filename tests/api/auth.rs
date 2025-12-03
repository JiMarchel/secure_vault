use crate::helpers::spawn_app;

#[tokio::test]
async fn test_sign_up_sends_otp_email() {
    let app = spawn_app().await;

    let response = app.sign_up("example".into(), "example@gmail.com".into()).await;

    assert_eq!(response.status(), 200);

    assert!(app.email_service.was_email_sent_to("example@gmail.com"));
    let otp = app.email_service.get_otp_for_email("example@gmail.com");
    assert!(otp.is_some());
    assert_eq!(otp.unwrap().len(), 6);
}

