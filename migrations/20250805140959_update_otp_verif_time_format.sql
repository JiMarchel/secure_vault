-- Add migration script here
ALTER TABLE otp_verif
ALTER COLUMN otp_expires_at TYPE TIMESTAMPTZ;