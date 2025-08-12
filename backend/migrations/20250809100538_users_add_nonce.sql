-- Add migration script here
ALTER TABLE users
ADD COLUMN nonce bytea;