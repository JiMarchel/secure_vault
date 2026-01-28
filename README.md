# Secure Vault

**Next-Generation Password Manager with Zero-Knowledge Architecture**

![Secure Vault Landing Page](docs/assets/landing_page.png)

## 🛡️ Project Overview

**Secure Vault** is a high-security password manager designed with a **Zero-Knowledge Architecture**. This means the server strictly manages encrypted data and **never** has access to the user's master password or the unencrypted content of their vault. All encryption and decryption operations occur client-side using WebAssembly (WASM) for maximum security and performance.

### 🔑 Key Features

- **Zero-Knowledge Privacy**: Your Master Password never leaves your device.
- **Client-Side Encryption**: Powered by Rust-compiled WASM for near-native performance.
- **Multi-Layer Encryption**:
  - **Layer 1 (Key Protection)**: Argon2id + HKDF-SHA256 protecting the Master Key.
  - **Layer 2 (Data Protection)**: XChaCha20-Poly1305 AEAD protecting your Vault Items.
- **Modern Security Standards**:
  - **Argon2id**: Memory-hard key derivation to resist GPU/ASIC brute-force attacks.
  - **XChaCha20-Poly1305**: High-performance authenticated encryption with extended 192-bit nonces.
  - **HKDF-SHA256**: Secure key derivation for authentication verification preventing timing attacks.
- **Robust Backend**: Built with Rust for memory safety and high concurrency.

---

## 🛠️ Technology Stack

### Frontend (Client)

- **Framework**: Nuxt 4 (Vue 3)
- **Language**: TypeScript
- **Styling**: Tailwind CSS v4, Shadcn-Vue
- **State Management**: Nuxt State / Composables
- **Forms**: TanStack Form + Zod Validation
- **Cryptography**: Custom WASM module compiled from Rust

### Backend (Server)

- **Language**: Rust
- **Web Framework**: Axum
- **Database**: PostgreSQL (via SQLx)
- **Caching & Rate Limiting**: Redis
- **Authentication**: JWT (Access + Refresh Tokens) with Token Family rotation
- **Logging**: Tracing

### Cryptography (WASM)

- **Crate**: `argon2`, `chacha20poly1305`, `hkdf`, `sha2`
- **Zeroization**: Sensitive memory is securely wiped after use.

---

## 🏗️ System Architecture

### Security Flow

1. **Registration**:
   - User generates `Master Key` locally via **Argon2id**.
   - `DEK` (Data Encryption Key) is generated and encrypted with the Master Key.
   - `Auth Verifier` is derived via **HKDF** and sent to the server for login verification.
   - **Result**: Server only stores the _encrypted_ DEK, Salt, and Auth Verifier.

2. **Login**:
   - User inputs Master Password.
   - Client requests encrypted DEK & Salt from server.
   - Client decrypts DEK locally. **If successful, the password is correct.**
   - Client sends Auth Verifier to server to obtain session tokens (JWT).

3. **Data Access**:
   - All Vault Items (passwords, notes) are encrypted/decrypted using the **DEK** and unique **Nonces**.
   - **XChaCha20-Poly1305** ensures data integrity with Authentication Tags.

### Directory Structure

```
secure_vault/
├── src/                  # Rust Backend (Axum)
│   ├── controller/       # API Handlers
│   ├── model/            # Database Models & Structs
│   ├── service/          # Business Logic
│   └── persistence/      # DB & Redis Access
├── ui/                   # Nuxt Frontend
│   ├── app/              # Vue Components & Pages
│   └── enc_dec/          # Rust WASM Encryption Module
├── migrations/           # SQL Database Migrations
└── docs/                 # Documentation
```
