CREATE EXTENSION IF NOT EXISTS "pgcrypto";

CREATE TABLE IF NOT EXISTS users (
    id SERIAL PRIMARY KEY,
    name VARCHAR(100) NOT NULL,
    email VARCHAR(100) NOT NULL,
    role VARCHAR(20) NOT NULL,
    password TEXT NOT NULL  
);

INSERT INTO users (name, email, role , password) VALUES
('Alice', 'alice@example.com', 'admin', crypt('password', gen_salt('bf'))),
('Bob', 'bob@example.com', 'standard', crypt('password', gen_salt('bf')))
