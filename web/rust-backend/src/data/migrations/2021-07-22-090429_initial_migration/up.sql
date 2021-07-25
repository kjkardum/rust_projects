CREATE TABLE users (
  id SERIAL PRIMARY KEY,
  username VARCHAR NOT NULL,
  password_hash TEXT NOT NULL,
  is_admin BOOLEAN NOT NULL DEFAULT 'f'
);

INSERT INTO users(username, password_hash, is_admin)
VALUES ('admin', 'None', TRUE)
RETURNING *;