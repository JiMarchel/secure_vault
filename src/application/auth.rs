// use std::sync::Arc;

// use tower_sessions::Session;
// use tracing::instrument;

// use crate::{model::{app_error::AppResult, response::SuccessResponse}, service::{jwt::JwtService, session::insert_session, user::UserPersistence}, validation::user::NewUserRequest};

// pub struct AuthUseCase {
//     user_persistence: Arc<dyn UserPersistence>,
//     jwt_service: Arc<JwtService>,
//     // otp_service: Arc<dyn OtpService>,
// }

// impl AuthUseCase {
//         #[instrument(
//         name= "use_case.sign_up",
//         skip(self, session, username, email),
//         fields(email=%email, username=%username)
//     )]
//     pub async fn sign_up(
//         &self,
//         username: &str,
//         email: &str,
//         session: Session,
//     ) -> AppResult<SuccessResponse<NewUserRequest>> {
//         if let Some(user_exists) = self.user_persistence.get_user_by_email(email).await? {
//             // return self.handle_existing_user(user_exists, session).await;
//         }

//         let user_id = self.user_persistence.create_user(username, email).await?;

//         self.send_verification_otp(user_id, email, username).await?;

//         insert_session(session, "verif_otp", user_id).await?;
//         Ok(SuccessResponse {
//             data: Some(NewUserRequest {
//                 username: username.to_string(),
//                 email: email.to_string(),
//             }),
//             message: "created".to_string(),
//         })
//     }
// }
