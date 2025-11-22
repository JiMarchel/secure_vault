use argon2::{
    Argon2, Params,
    password_hash::{SaltString, rand_core},
};
use base64::{Engine, prelude::BASE64_STANDARD};
use chacha20poly1305::{
    KeyInit, XChaCha20Poly1305, XNonce,
    aead::{Aead, Payload},
};
use serde::{Deserialize, Serialize};
use wasm_bindgen::prelude::*;
use zeroize::Zeroize;

// cargo install wasm-pack
// wasm-pack build --release --target web --out-dir pkg

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UserIdentifier {
    pub encrypted_dek: String,
    pub nonce: String,
    pub salt: String,
    pub argon2_params: String,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct UnlockResult {
    pub success: bool,
}

/// AAD (Authenticated Additional Data) bytes for the vault.
fn aad_bytes() -> &'static [u8] {
    b"aead=xchacha20poly1305|kdf=argon2id"
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


/// Parse Argon2 params dari string
fn parse_argon2_params(param_str: &str) -> Result<Params, String> {
    let mut m_cost = 65536;  // Default: 64 MB
    let mut t_cost = 3;      // Default: 3 iterations
    let mut p_cost = 1;      // Default: 1 thread
    
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
    
    Params::new(m_cost, t_cost, p_cost, None)
        .map_err(|e| e.to_string())
}

fn derive_master_key(master_password: &str, salt: &[u8], params: Params) -> [u8; 32] {
    let argon2 = Argon2::new(argon2::Algorithm::Argon2id, argon2::Version::V0x13, params);
    let mut key = [0u8; 32];
    argon2
        .hash_password_into(master_password.as_bytes(), salt, &mut key)
        .expect("Argon2 derive failed");

    key
}



#[wasm_bindgen]
pub fn create_user_identifier(master_password: &str) -> String {
    let param = Params::new(64 * 1024, 3, 1, None).expect("Invalid argon2 params");
    let argon2_param_str = format_argon2_params(&param);

    let salt = SaltString::generate(&mut rand_core::OsRng);
    let salt_raw = salt.as_salt().as_str().as_bytes();

    let master_key = derive_master_key(master_password, salt_raw, param);

    let mut dek = [0u8; 32];
    getrandom::getrandom(&mut dek).expect("RNG failed");

    let chiper = XChaCha20Poly1305::new(&master_key.into());
    let mut nonce_bytes = [0u8; 24];
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

    let vp = UserIdentifier {
        encrypted_dek: BASE64_STANDARD.encode(&encrypted),
        nonce: BASE64_STANDARD.encode(&nonce_bytes),
        salt: BASE64_STANDARD.encode(salt_raw),
        argon2_params: argon2_param_str,
    };

    serde_json::to_string(&vp).expect("Serialize failed")
}


/// FUNCTION 2: Unlock Vault (Login)
/// 
/// Dipanggil saat user login dengan master password.
/// 
/// Input: 
/// - master_password: Password yang diinput user
/// - vault_data_json: Data vault dari backend (JSON string)
/// 
/// Output: JSON string berisi {success, dek}
/// - success: true jika password benar, false jika salah
/// - dek: Base64 encoded DEK (jika success)
/// 
/// Process:
/// 1. Parse vault data dari backend
/// 2. Derive master key dari password + salt
/// 3. Try decrypt DEK dengan master key
/// 4. Jika decrypt berhasil = password benar ✅
/// 5. Jika decrypt gagal = password salah ❌
#[wasm_bindgen]
pub fn unlock_vault(master_password: &str, vault_data_json: &str) -> String {
    // 1. Parse vault data dari JSON
    let vault_data: UserIdentifier = match serde_json::from_str(vault_data_json) {
        Ok(data) => data,
        Err(_) => {
            return serde_json::to_string(&UnlockResult {
                success: false,
            }).unwrap();
        }
    };
    
    // 2. Decode Base64 strings
    let encrypted_dek = match BASE64_STANDARD.decode(&vault_data.encrypted_dek) {
        Ok(data) => data,
        Err(_) => {
            return serde_json::to_string(&UnlockResult {
                success: false,
            }).unwrap();
        }
    };
    
    let nonce_bytes = match BASE64_STANDARD.decode(&vault_data.nonce) {
        Ok(data) => data,
        Err(_) => {
            return serde_json::to_string(&UnlockResult {
                success: false,
            }).unwrap();
        }
    };
    
    let salt_bytes = match BASE64_STANDARD.decode(&vault_data.salt) {
        Ok(data) => data,
        Err(_) => {
            return serde_json::to_string(&UnlockResult {
                success: false,
            }).unwrap();
        }
    };
    
    // 3. Parse Argon2 parameters
    let params = match parse_argon2_params(&vault_data.argon2_params) {
        Ok(p) => p,
        Err(_) => {
            return serde_json::to_string(&UnlockResult {
                success: false,
            }).unwrap();
        }
    };
    
    // 4. Derive master key (sama seperti saat create)
    let master_key = derive_master_key(master_password, &salt_bytes, params);
    
    // 5. Try decrypt DEK
    let cipher = XChaCha20Poly1305::new(&master_key.into());
    let nonce = XNonce::from_slice(&nonce_bytes);
    
    match cipher.decrypt(nonce, encrypted_dek.as_slice()) {
        Ok(_) => (),
        Err(_) => {
            // ❌ Decrypt gagal = password SALAH!
            let mut key_for_zeroize = master_key;
            key_for_zeroize.zeroize();
            
            return serde_json::to_string(&UnlockResult {
                success: false,
            }).unwrap();
        }
    };
    
    // ✅ Success! Password benar, DEK berhasil di-decrypt
    // let dek_base64 = BASE64_STANDARD.encode(&decrypted);
    
    // Zeroize sensitive data
    let mut key_for_zeroize = master_key;
    key_for_zeroize.zeroize();
    
    serde_json::to_string(&UnlockResult {
        success: true,
    }).unwrap()
}