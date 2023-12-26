CREATE TABLE wishlists (
    id SERIAL PRIMARY KEY,
    title VARCHAR NOT NULL,
    description VARCHAR,
    date TIMESTAMP,
    user_id INTEGER NOT NULL REFERENCES users(id),
    published BOOLEAN NOT NULL DEFAULT FALSE
);