-- Add migration script here
CREATE TABLE IF NOT EXISTS users
(
    id              BIGSERIAL PRIMARY KEY,
    name            TEXT NOT NULL,
    password_digest TEXT NOT NULL
);
