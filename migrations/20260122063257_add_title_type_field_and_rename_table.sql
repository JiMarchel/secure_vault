-- Add migration script here
ALTER TABLE credentials RENAME TO vaults;
ALTER TABLE vaults ADD COLUMN title VARCHAR(100) NOT NULL;
ALTER TABLE vaults ADD COLUMN item_type VARCHAR(20) NOT NULL;