CREATE TABLE products (
  id SERIAL PRIMARY KEY,
  name VARCHAR NOT NULL,
  description TEXT,
  url VARCHAR,
  price REAL NOT NULL,
  available BOOLEAN NOT NULL DEFAULT TRUE
);
