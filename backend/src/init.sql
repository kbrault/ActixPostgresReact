CREATE EXTENSION IF NOT EXISTS "pgcrypto";

CREATE TABLE IF NOT EXISTS users (
    id SERIAL PRIMARY KEY,
    name VARCHAR(100) NOT NULL,
    age INT NOT NULL,
    email VARCHAR(100) NOT NULL,
    password TEXT NOT NULL  
);

INSERT INTO users (name, age, email, password) VALUES
('Alice', 30, 'alice@example.com', crypt('alicepassword', gen_salt('bf'))),
('Bob', 25, 'bob@example.com', crypt('bobpassword', gen_salt('bf'))),
('Charlie', 35, 'charlie@example.com', crypt('charliepassword', gen_salt('bf')));
