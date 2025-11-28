-- Aktifkan ekstensi untuk menghasilkan UUID jika belum ada
-- Jalankan perintah ini sekali per database
CREATE EXTENSION IF NOT EXISTS "uuid-ossp";

CREATE TABLE "users" (
  "id" uuid PRIMARY KEY DEFAULT uuid_generate_v4(),
  "username" VARCHAR(255),
  "email" VARCHAR(255) UNIQUE NOT NULL,
  
  -- Kolom untuk vault 
  "encrypted_DEK" BYTEA,
  "salt" BYTEA,
  "argon2_params" TEXT,
  
  -- verifikasi email
  "is_email_verified" BOOLEAN DEFAULT false,
  "otp_code" VARCHAR(6),
  "otp_expires_at" TIMESTAMP,
  
  -- Timestamps
  "created_at" TIMESTAMP DEFAULT CURRENT_TIMESTAMP
);

CREATE TABLE "credentials" (
  "id" uuid PRIMARY KEY DEFAULT uuid_generate_v4(),
  "user_id" uuid NOT NULL REFERENCES "users"("id") ON DELETE CASCADE,
  "name" VARCHAR(255) NOT NULL,
  "encrypted_data" BYTEA NOT NULL,
  "nonce" BYTEA NOT NULL,
  
  -- Timestamps
  "created_at" TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
  "updated_at" TIMESTAMP DEFAULT CURRENT_TIMESTAMP
);

-- Membuat index pada kolom user_id di tabel credentials untuk mempercepat query
CREATE INDEX "idx_credentials_user_id" ON "credentials"("user_id");

-- Trigger untuk memperbarui kolom updated_at secara otomatis
-- Ini adalah contoh untuk PostgreSQL
CREATE OR REPLACE FUNCTION update_updated_at_column()
RETURNS TRIGGER AS $$
BEGIN
   NEW.updated_at = now(); 
   RETURN NEW;
END;
$$ language 'plpgsql';

CREATE TRIGGER update_credentials_updated_at
BEFORE UPDATE ON "credentials"
FOR EACH ROW
EXECUTE FUNCTION update_updated_at_column();

