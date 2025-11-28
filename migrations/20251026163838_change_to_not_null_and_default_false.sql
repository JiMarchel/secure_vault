-- Add migration script here
ALTER TABLE users
  ALTER COLUMN is_email_verified SET DEFAULT false,
  ALTER COLUMN is_email_verified SET NOT NULL;