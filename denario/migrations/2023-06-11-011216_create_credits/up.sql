-- Your SQL goes here
CREATE TABLE credits (
  id SERIAL PRIMARY KEY,
  name VARCHAR(50) NOT NULL,
  comment TEXT NULL,
  amount FLOAT NOT NULL,
  payments INTEGER NOT NULL,
  started_at DATE NOT NULL,
  finish_at DATE NOT NULL,
  category_id INTEGER NOT NULL REFERENCES categories(id),
  created_at TIMESTAMP NOT NULL DEFAULT CURRENT_DATE,
  updated_at TIMESTAMP NOT NULL DEFAULT CURRENT_DATE,
  is_deleted BOOLEAN NOT NULL DEFAULT FALSE
)