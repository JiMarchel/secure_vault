-- Add migration script here
CREATE INDEX idx_vaults_user_title ON vaults(user_id, LOWER(title));