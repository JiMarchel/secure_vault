-- Add migration script here
ALTER TABLE users
ALTER COLUMN username SET NOT NULL;