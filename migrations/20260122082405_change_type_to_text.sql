-- Add migration script here
ALTER TABLE vaults ALTER COLUMN encrypted_data TYPE text;
ALTER TABLE vaults ALTER COLUMN nonce TYPE text;