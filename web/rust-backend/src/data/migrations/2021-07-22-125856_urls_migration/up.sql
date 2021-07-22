CREATE TABLE urls (
  id SERIAL PRIMARY KEY,
  user_id INT,
  long_url VARCHAR NOT NULL,
  short_url TEXT NOT NULL,
  uses INT NOT NULL DEFAULT 0,
  CONSTRAINT fk_user
    FOREIGN KEY(user_id) 
      REFERENCES users (id)
      ON DELETE CASCADE
);