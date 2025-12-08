use std::sync::Arc;

use tracing::instrument;
use uuid::Uuid;

use crate::{
    model::{
        app_error::AppResult,
        otp::{OtpExpiresAt, OtpRecord},
        response::SuccessResponse,
    },
    service::otp::OtpService,
};

pub struct OtpUseCase {
    pub otp_service: Arc<OtpService>,
}

impl OtpUseCase {
    pub fn new(otp_service: Arc<OtpService>) -> Self {
        Self { otp_service }
    }

    #[instrument(
        name = "use_case.send_verification_otp",
        skip(self, user_id, username, email)
    )]
    pub async fn send_verification_otp(
        &self,
        user_id: Uuid,
        username: &str,
        email: &str,
    ) -> AppResult<SuccessResponse<()>> {
        self.otp_service
            .send_verification(user_id, username, email)
            .await?;

        Ok(SuccessResponse {
            data: None,
            message: "Success send otp".into(),
        })
    }

    #[instrument(
        name = "use_case.resend_verification_otp",
        skip(self, user_id, username, email)
    )]
    pub async fn resend_verification_otp(
        &self,
        user_id: Uuid,
        username: &str,
        email: &str,
    ) -> AppResult<SuccessResponse<()>> {
        self.otp_service
            .resend_verification(user_id, username, email)
            .await?;

        Ok(SuccessResponse {
            data: None,
            message: "Success resend otp".into(),
        })
    }

    #[instrument(name = "use_case.get_otp_by_user_id", skip(self, user_id))]
    pub async fn get_otp_by_user_id(&self, user_id: Uuid) -> AppResult<SuccessResponse<OtpRecord>> {
        let otp_record = self.otp_service.get_otp_by_user_id(user_id).await?;

        Ok(SuccessResponse {
            data: Some(otp_record),
            message: "Success get otp by user id".into(),
        })
    }

    #[instrument(name = "use_case.get_otp_expire_by_user_id", skip(self, user_id))]
    pub async fn get_otp_expire_by_user_id(
        &self,
        user_id: Uuid,
    ) -> AppResult<SuccessResponse<OtpExpiresAt>> {
        let otp_expires = self.otp_service.get_otp_expire_by_user_id(user_id).await?;

        Ok(SuccessResponse {
            data: Some(otp_expires),
            message: "Success get otp expires".to_string(),
        })
    }
}
