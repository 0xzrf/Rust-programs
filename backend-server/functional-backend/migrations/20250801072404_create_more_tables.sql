ALTER TABLE users DROP CONSTRAINT IF EXISTS users_username_key;

ALTER TABLE users ADD PRIMARY KEY (username);

CREATE TABLE contacts (
    id SERIAL PRIMARY KEY,
    email TEXT NOT NULL,
    phone TEXT,
    username TEXT NOT NULL REFERENCES users(username)
);
