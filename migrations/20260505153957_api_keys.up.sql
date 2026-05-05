-- Add up migration script here
CREATE TABLE api_keys (
    id               UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    user_id          UUID NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    exchange         TEXT NOT NULL, -- On y mettra "CRYPTOTCOM"
    encrypted_key    BYTEA NOT NULL,
    encrypted_secret BYTEA NOT NULL,
    created_at       TIMESTAMPTZ NOT NULL DEFAULT now()
);