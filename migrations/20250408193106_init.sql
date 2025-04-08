-- Add migration script here
CREATE TABLE links (
    id SERIAL PRIMARY KEY,
    target_url TEXT NOT NULL,
    shortened_url TEXT NOT NULL,
    created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP
);