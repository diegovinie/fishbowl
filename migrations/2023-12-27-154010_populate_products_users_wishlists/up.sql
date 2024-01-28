-- Your SQL goes here
INSERT INTO users (name, email, password, role, active) VALUES
    ('Claudia', 'claudia@dummy.test', decode('8776f108e247ab1e2b323042c049c266407c81fbad41bde1e8dfc1bb66fd267e', 'hex'), 'ADMIN', TRUE),
    ('Napoleon', 'napoleon@dummy.test', decode('8776f108e247ab1e2b323042c049c266407c81fbad41bde1e8dfc1bb66fd267e', 'hex'), 'USER', TRUE);
