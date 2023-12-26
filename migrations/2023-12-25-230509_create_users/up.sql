CREATE TABLE users (
  id SERIAL PRIMARY KEY,
  name VARCHAR NOT NULL,
  email TEXT NOT NULL,
  password TEXT NOT NULL,
  active BOOLEAN NOT NULL DEFAULT FALSE
);

INSERT INTO users (name, email, password)
    VALUES ('Claudia', 'claudia@dummy.test', 'password');