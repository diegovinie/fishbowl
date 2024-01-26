-- Your SQL goes here
-- INSERT INTO products (name, description, url, price) VALUES
--     ('Armario Amarillo', 'El armario de mi abuela', 'https://t.ly/cel6a', 850000),
--     ('Comedor Dorado', 'El comedor de mi abuela', null, 500000),
--     ('Cuna ACME', 'La mejor cuna', null, 250000),
--     ('Licuadora Oster', 'La mejor cuna', 'https://t.ly/eKctR', 234900);

INSERT INTO users (name, email, password, role, active) VALUES
    ('Claudia', 'claudia@dummy.test', decode('8776f108e247ab1e2b323042c049c266407c81fbad41bde1e8dfc1bb66fd267e', 'hex'), 'ADMIN', TRUE),
    ('Napoleon', 'napoleon@dummy.test', decode('8776f108e247ab1e2b323042c049c266407c81fbad41bde1e8dfc1bb66fd267e', 'hex'), 'USER', TRUE);

INSERT INTO wishlists (title, description, date, user_id, published) VALUES
    ('Mi boda', 'Esta es mi boda', '2024-03-17 20:38:16', 1, TRUE),
    ('Babyshower de Maria', null, null, 2, TRUE),
    ('Draft de mi boda', 'TODO', null, 1, FALSE);