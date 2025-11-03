-- Rename column from encrypted_DEK to encrypted_dek
ALTER TABLE users
RENAME COLUMN "encrypted_DEK" TO encrypted_dek;