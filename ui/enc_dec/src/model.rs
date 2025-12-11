use serde::{Deserialize, Serialize};

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct WasmResponse<T> {
    pub success: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<T>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<String>,
}

impl<T> WasmResponse<T> {
    pub fn success(data: T) -> Self {
        WasmResponse {
            success: true,
            data: Some(data),
            error: None,
        }
    }

    pub fn error(err_msg: String) -> Self {
        WasmResponse {
            success: false,
            data: None,
            error: Some(err_msg),
        }
    }
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UserIdentifier {
    pub encrypted_dek: String,
    pub nonce: String,
    pub salt: String,
    pub argon2_params: String,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LoginData {
    pub dek: String,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct VaultItem {
    pub encrypted_data: String,
    pub nonce: String,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DecryptedVaultItem {
    pub plaintext: String,
}
