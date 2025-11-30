use crate::helpers::spawn_app;

#[tokio::test]
async fn test_sign_up_sends_otp_email() {
    let app = spawn_app().await;

    let response = app.sign_up("wkwk".into(), "jmarchel200@gmail.com".into()).await;

    assert_eq!(response.status(), 200)
}