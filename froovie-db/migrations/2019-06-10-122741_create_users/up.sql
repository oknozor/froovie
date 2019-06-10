CREATE TABLE users
(
    id    SERIAL PRIMARY KEY,
    nick  TEXT NOT NULL,
    email TEXT NOT NULL,
    password_hash TEXT NOT NULL
)