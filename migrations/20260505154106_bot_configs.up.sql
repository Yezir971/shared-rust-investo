-- Add up migration script here
CREATE TABLE bot_configs (
    id               UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    user_id          UUID NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    pair             TEXT NOT NULL DEFAULT 'BTC_USDT',
    strategy         TEXT NOT NULL,
    interval_secs    INTEGER NOT NULL DEFAULT 900,
    max_position_pct NUMERIC NOT NULL DEFAULT 0.1,
    active           BOOLEAN NOT NULL DEFAULT false,
    updated_at       TIMESTAMPTZ NOT NULL DEFAULT now()
);