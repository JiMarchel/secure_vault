-- Add token_family and is_revoked for refresh token rotation
ALTER TABLE refresh_tokens ADD COLUMN token_family UUID DEFAULT gen_random_uuid();
ALTER TABLE refresh_tokens ADD COLUMN is_revoked BOOLEAN DEFAULT FALSE;
CREATE INDEX idx_refresh_tokens_token_family ON refresh_tokens(token_family);
