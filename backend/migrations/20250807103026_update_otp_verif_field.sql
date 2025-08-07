-- Add migration script here
ALTER TABLE otp_verif
ALTER COLUMN otp_code SET NOT NULL;

ALTER TABLE otp_verif
ALTER COLUMN otp_expires_at SET NOT NULL;