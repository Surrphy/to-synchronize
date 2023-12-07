-- Add migration script here
CREATE TABLE IF NOT EXISTS todos (
  user_id INT PRIMARY KEY NOT NULL,
  todos BYTEA
);
