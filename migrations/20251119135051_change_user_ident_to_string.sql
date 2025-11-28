-- Add migration script here
ALTER TABLE users
ALTER COLUMN encrypted_dek TYPE TEXT,
ALTER COLUMN nonce TYPE TEXT,
ALTER COLUMN salt TYPE TEXT;
