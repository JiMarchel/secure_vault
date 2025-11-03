-- change nullable to not null
ALTER TABLE users
ALTER COLUMN created_at SET NOT NULL;
