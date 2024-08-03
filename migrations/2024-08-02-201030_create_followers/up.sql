CREATE TABLE followers (
    id SERIAL PRIMARY KEY,
    wishlist_id INTEGER NOT NULL,
    FOREIGN KEY(wishlist_id) REFERENCES wishlists(id) ON DELETE CASCADE,
    user_id INTEGER NOT NULL,
    FOREIGN KEY(user_id) REFERENCES users(id) ON DELETE CASCADE,
    active BOOLEAN NOT NULL DEFAULT false
);