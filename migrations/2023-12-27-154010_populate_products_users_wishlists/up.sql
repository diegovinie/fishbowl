-- Your SQL goes here
-- INSERT INTO products (name, description, url, price) VALUES
--     ('Armario Amarillo', 'El armario de mi abuela', 'https://t.ly/cel6a', 850000),
--     ('Comedor Dorado', 'El comedor de mi abuela', null, 500000),
--     ('Cuna ACME', 'La mejor cuna', null, 250000),
--     ('Licuadora Oster', 'La mejor cuna', 'https://t.ly/eKctR', 234900);

INSERT INTO users (name, email, password, active) VALUES
    ('Claudia', 'claudia@dummy.test', 'password', TRUE),
    ('Napoleon', 'napoleon@dummy.test', 'password', TRUE),
    ('San Martin', 'martin@dummy.test', 'password', FALSE),
    ('Madonna', 'madona@dummy.test', 'password', FALSE);

INSERT INTO wishlists (title, description, date, user_id, published) VALUES
    ('Mi boda', 'Esta es mi boda', '2024-03-17 20:38:16', 1, TRUE),
    ('Babyshower de Maria', null, null, 2, FALSE);