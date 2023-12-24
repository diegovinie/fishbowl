CREATE TABLE products (
  id SERIAL PRIMARY KEY,
  name VARCHAR NOT NULL,
  description TEXT NOT NULL,
  price REAL NOT NULL,
  available BOOLEAN NOT NULL DEFAULT TRUE
);

INSERT INTO products (name, description, price)
    VALUES ('Armario', 'El armario de mi abuela', 850000);