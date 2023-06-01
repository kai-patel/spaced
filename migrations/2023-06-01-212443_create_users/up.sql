-- Your SQL goes here

CREATE TABLE users (
    id SERIAL PRIMARY KEY,
    name TEXT NOT NULL,
    display_name TEXT NOT NULL,
    password_hash TEXT,
    email TEXT,
    federation_id TEXT NOT NULL,
    inbox TEXT NOT NULL,
    outbox TEXT NOT NULL,
    local BOOLEAN NOT NULL DEFAULT FALSE,
    public_key TEXT NOT NULL,
    private_key TEXT,
    last_refreshed_at TIMESTAMPTZ NOT NULL
)
