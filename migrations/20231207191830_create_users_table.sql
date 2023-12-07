-- Add migration script here
CREATE TABLE IF NOT EXISTS users (
  id SERIAL PRIMARY KEY NOT NULL,
  username TEXT UNIQUE NOT NULL,
  password TEXT NOT NULL
);
