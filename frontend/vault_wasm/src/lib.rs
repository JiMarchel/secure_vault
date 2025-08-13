use argon2::{
    Argon2, Params,
    password_hash::{SaltString, rand_core},
};
use base64::{Engine, prelude::BASE64_STANDARD};
use chacha20poly1305::{
    Key, KeyInit, XChaCha20Poly1305, XNonce,
    aead::{Aead, Payload},
};
use serde::Serialize;
use wasm_bindgen::prelude::*;
use zeroize::Zeroize;

#[derive(Serialize)]
pub struct VaultPayload {
    pub encrypted_dek: String,
    pub nonce: String,
    pub salt: String,
    pub argon2_params: String,
}

fn format_argon2_params(p: &Params) -> String {
    format!(
        "algo=argon2id,v={},m={},t={},p={}",
        19,
        p.m_cost(),
        p.t_cost(),
        p.p_cost()
    )
}

fn derive_master_key(master_password: &str, salt: &[u8], params: Params) -> [u8; 32] {
    let argon2 = Argon2::new(argon2::Algorithm::Argon2id, argon2::Version::V0x13, params);
    let mut key = [0u8; 32];
    argon2
        .hash_password_into(master_password.as_bytes(), salt, &mut key)
        .expect("Argon2 derive failed");

    key
}

/// AAD (Authenticated Additional Data) bytes for the vault.
fn aad_bytes() -> &'static [u8] {
    b"aead=xchacha20poly1305|kdf=argon2id"
}

#[wasm_bindgen]
pub fn create_vault_wasm(master_password: &str) -> String {
    let param = Params::new(64 * 1024, 3, 1, None).expect("Invalid argon2 params");
    let argon2_param_str = format_argon2_params(&param);

    let salt = SaltString::generate(&mut rand_core::OsRng);
    let salt_raw = salt.as_salt().as_str().as_bytes();

    let master_key = derive_master_key(master_password, salt_raw, param);

    let mut dek = [0u8; 32];
    getrandom::getrandom(&mut dek).expect("RNG failed");

    let chiper = XChaCha20Poly1305::new(Key::from_slice(&master_key));
    let mut nonce_bytes = [0u8, 24];
    getrandom::getrandom(&mut nonce_bytes).expect("RNG failed");
    let nonce = XNonce::from_slice(&nonce_bytes);

    let payload = Payload {
        msg: &dek,
        aad: aad_bytes(),
    };
    let encrypted = chiper.encrypt(nonce, payload).expect("Encrypt failed");

    let mut key_for_zeroize = master_key;
    key_for_zeroize.zeroize();
    dek.zeroize();

    let vp = VaultPayload {
        encrypted_dek: BASE64_STANDARD.encode(&encrypted),
        nonce: BASE64_STANDARD.encode(&nonce_bytes),
        salt: BASE64_STANDARD.encode(salt_raw),
        argon2_params: argon2_param_str,
    };

    serde_json::to_string(&vp).expect("Serialize failed")
}
