-- Add migration script here
CREATE TABLE otp_verif(
    "user_id" uuid NOT NULL REFERENCES "users"("id") ON DELETE CASCADE,
    "otp_code" VARCHAR(6),
    "otp_expires_at" TIMESTAMP
);
