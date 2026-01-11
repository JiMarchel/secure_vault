-- Add auth_verifier column for zero-knowledge authentication
-- This stores a HKDF-derived verifier that proves password knowledge
ALTER TABLE users ADD COLUMN auth_verifier TEXT;
