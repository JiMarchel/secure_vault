use serde::Serialize;
use wasm_bindgen::prelude::*;

use crate::{
    model::{DecryptedVaultItem, LoginData, UserIdentifier, VaultItem, WasmResponse},
    user::{decrypt_user_identifier_internal, encrypt_user_identifier_internal},
    vault::{decrypt_vault_item_internal, encrypt_vault_item_internal},
};

pub mod error;
pub mod model;
pub mod user;
pub mod vault;

// cargo install wasm-pack
// wasm-pack build --release --target web --out-dir pkg

fn to_json_string<T: Serialize>(response: WasmResponse<T>) -> String {
    serde_json::to_string(&response).unwrap_or_else(|_| {
        serde_json::to_string(&WasmResponse::<()>::error(
            "Failed to serialize response".to_string(),
        ))
        .unwrap()
    })
}

#[wasm_bindgen]
pub fn encrypt_user_identifier(master_password: &str) -> String {
    match encrypt_user_identifier_internal(master_password) {
        Ok(data) => to_json_string(WasmResponse::success(data)),
        Err(e) => to_json_string(WasmResponse::<UserIdentifier>::error(e.to_string())),
    }
}

#[wasm_bindgen]
pub fn decrypt_user_identifier(master_password: &str, vault_data_json: &str) -> String {
    match decrypt_user_identifier_internal(master_password, vault_data_json) {
        Ok((dek, auth_verifier)) => {
            to_json_string(WasmResponse::success(LoginData { dek, auth_verifier }))
        }
        Err(e) => to_json_string(WasmResponse::<LoginData>::error(e.to_string())),
    }
}

#[wasm_bindgen]
pub fn encrypt_vault_item(dek_base64: &str, plaintext: &str) -> String {
    match encrypt_vault_item_internal(dek_base64, plaintext) {
        Ok(data) => to_json_string(WasmResponse::success(data)),
        Err(e) => to_json_string(WasmResponse::<VaultItem>::error(e.to_string())),
    }
}

#[wasm_bindgen]
pub fn decrypt_vault_item(dek_base64: &str, vault_item_json: &str) -> String {
    match decrypt_vault_item_internal(dek_base64, vault_item_json) {
        Ok(plaintext) => to_json_string(WasmResponse::success(DecryptedVaultItem { plaintext })),
        Err(e) => to_json_string(WasmResponse::<DecryptedVaultItem>::error(e.to_string())),
    }
}
