-- Add migration script here
ALTER TABLE links ADD UNIQUE (shortened_url);