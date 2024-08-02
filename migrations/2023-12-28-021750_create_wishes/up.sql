CREATE TABLE wishes (
    id SERIAL PRIMARY KEY,
    pending BOOLEAN NOT NULL DEFAULT TRUE,
    wishlist_id INTEGER NOT NULL,
    FOREIGN KEY(wishlist_id) REFERENCES wishlists(id) ON DELETE CASCADE,
    product_id INTEGER NOT NULL REFERENCES products(id)
);

-- INSERT INTO wishes (wishlist_id, product_id) VALUES
--     (1, 1),
--     (1, 2),
--     (1, 3);