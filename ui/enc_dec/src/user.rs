use argon2::{
    Argon2, Params,
    password_hash::{SaltString, rand_core},
};
use base64::{Engine, prelude::BASE64_STANDARD};
use chacha20poly1305::{
    KeyInit, XChaCha20Poly1305, XNonce,
    aead::{Aead, Payload},
};
use zeroize::Zeroize;

use crate::{
    error::{AppResult, VaultError},
    model::UserIdentifier,
};

fn format_argon2_params(p: &Params) -> String {
    format!(
        "algo=argon2id,v={},m={},t={},p={}",
        19,
        p.m_cost(),
        p.t_cost(),
        p.p_cost()
    )
}

fn aad_dek() -> &'static [u8] {
    b"vault_v1|type=dek|aead=xchacha20poly1305|kdf=argon2id"
}

/// Parse Argon2 params dari string
fn parse_argon2_params(param_str: &str) -> Result<Params, String> {
    let mut m_cost = 65536; // Default: 64 MB
    let mut t_cost = 3; // Default: 3 iterations
    let mut p_cost = 1; // Default: 1 thread

    for part in param_str.split(',') {
        if let Some((key, value)) = part.split_once('=') {
            match key {
                "m" => m_cost = value.parse().unwrap_or(65536),
                "t" => t_cost = value.parse().unwrap_or(3),
                "p" => p_cost = value.parse().unwrap_or(1),
                _ => {}
            }
        }
    }

    Params::new(m_cost, t_cost, p_cost, None).map_err(|e| e.to_string())
}

fn derive_master_key(master_password: &str, salt: &[u8], params: Params) -> [u8; 32] {
    let argon2 = Argon2::new(argon2::Algorithm::Argon2id, argon2::Version::V0x13, params);
    let mut key = [0u8; 32];
    argon2
        .hash_password_into(master_password.as_bytes(), salt, &mut key)
        .expect("Argon2 derive failed");

    key
}

pub fn encrypt_user_identifier_internal(master_password: &str) -> AppResult<UserIdentifier> {
    let param =
        Params::new(64 * 1024, 3, 1, None).map_err(|e| VaultError::InvalidParams(e.to_string()))?;
    let argon2_param_str = format_argon2_params(&param);

    let salt = SaltString::generate(&mut rand_core::OsRng);
    let salt_raw = salt.as_salt().as_str().as_bytes();

    let master_key = derive_master_key(master_password, salt_raw, param);

    let mut dek = [0u8; 32];
    getrandom::getrandom(&mut dek).map_err(|e| VaultError::RngError(e.to_string()))?;

    let chiper = XChaCha20Poly1305::new(&master_key.into());
    let mut nonce_bytes = [0u8; 24];
    getrandom::getrandom(&mut nonce_bytes).map_err(|e| VaultError::RngError(e.to_string()))?;
    let nonce = XNonce::from_slice(&nonce_bytes);

    let payload = Payload {
        msg: &dek,
        aad: aad_dek(),
    };

    let encrypted = chiper
        .encrypt(nonce, payload)
        .map_err(|_| VaultError::EncryptionFailed)?;

    let mut key_for_zeroize = master_key;
    key_for_zeroize.zeroize();
    dek.zeroize();

    Ok(UserIdentifier {
        encrypted_dek: BASE64_STANDARD.encode(&encrypted),
        nonce: BASE64_STANDARD.encode(&nonce_bytes),
        salt: BASE64_STANDARD.encode(salt_raw),
        argon2_params: argon2_param_str,
    })
}


pub fn decrypt_user_identifier_internal(master_password: &str, vault_data_json: &str) -> AppResult<String> {
    let vault_data: UserIdentifier = serde_json::from_str(vault_data_json)?;

    let encrypted_dek = BASE64_STANDARD.decode(&vault_data.encrypted_dek)?;
    let nonce_bytes = BASE64_STANDARD.decode(&vault_data.nonce)?;
    let salt_bytes = BASE64_STANDARD.decode(&vault_data.salt)?;

    let params = parse_argon2_params(&vault_data.argon2_params)
        .map_err(|e| VaultError::InvalidParams(e))?;

    let master_key = derive_master_key(master_password, &salt_bytes, params);

    let chiper = XChaCha20Poly1305::new(&master_key.into());
    let nonce = XNonce::from_slice(&nonce_bytes);

    let payload = Payload {
        msg: &encrypted_dek,
        aad: aad_dek(),
    };

    let dek = chiper
        .decrypt(nonce, payload)
        .map_err(|_| VaultError::DecryptionFailed)?;

    let mut key_for_zeroize = master_key;
    key_for_zeroize.zeroize();

    Ok(BASE64_STANDARD.encode(&dek))
}