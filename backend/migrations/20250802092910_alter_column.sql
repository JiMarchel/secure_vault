-- Add migration script here
ALTER TABLE users
DROP COLUMN otp_code;

ALTER TABLE users
DROP COLUMN otp_expires_at;
