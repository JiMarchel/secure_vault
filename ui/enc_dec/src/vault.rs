use base64::{Engine, prelude::BASE64_STANDARD};
use chacha20poly1305::{
    KeyInit, XChaCha20Poly1305, XNonce,
    aead::{Aead, Payload},
};

use crate::{
    error::{AppResult, VaultError},
    model::VaultItem,
};

fn aad_vault_item() -> &'static [u8] {
    b"vault_v1|type=item|aead=xchacha20poly1305"
}

pub fn encrypt_vault_item_internal(dek_base64: &str, plaintext: &str) -> AppResult<VaultItem> {
    let dek = BASE64_STANDARD.decode(dek_base64)?;
    let cipher = XChaCha20Poly1305::new(dek.as_slice().into());

    let mut nonce_bytes = [0u8; 24];
    getrandom::getrandom(&mut nonce_bytes).map_err(|e| VaultError::RngError(e.to_string()))?;
    let nonce = XNonce::from_slice(&nonce_bytes);

    let payload = Payload {
        msg: plaintext.as_bytes(),
        aad: aad_vault_item(),
    };

    let encrypted = cipher
        .encrypt(nonce, payload)
        .map_err(|_| VaultError::EncryptionFailed)?;

    Ok(VaultItem {
        encrypted_data: BASE64_STANDARD.encode(&encrypted),
        nonce: BASE64_STANDARD.encode(&nonce_bytes),
    })
}

pub fn decrypt_vault_item_internal(dek_base64: &str, vault_item_json: &str) -> AppResult<String> {
    let item: VaultItem = serde_json::from_str(vault_item_json)?;

    let dek = BASE64_STANDARD.decode(dek_base64)?;
    let encrypted = BASE64_STANDARD.decode(&item.encrypted_data)?;
    let nonce_bytes = BASE64_STANDARD.decode(&item.nonce)?;

    let cipher = XChaCha20Poly1305::new(dek.as_slice().into());
    let nonce = XNonce::from_slice(&nonce_bytes);

    let payload = Payload {
        msg: encrypted.as_slice(),
        aad: aad_vault_item(),
    };

    let decrypted = cipher
        .decrypt(nonce, payload)
        .map_err(|_| VaultError::DecryptionFailed)?;

    String::from_utf8(decrypted).map_err(|_| VaultError::DecryptionFailed)
}
