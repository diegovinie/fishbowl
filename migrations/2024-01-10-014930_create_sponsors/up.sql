CREATE TABLE sponsors (
    id SERIAL PRIMARY KEY,
    amount REAL NOT NULL,
    leader BOOLEAN DEFAULT FALSE,
    wish_id INTEGER NOT NULL REFERENCES wishes(id),
    user_id INTEGER NOT NULL REFERENCES users(id)
);
