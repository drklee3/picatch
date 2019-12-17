CREATE TABLE users (
    id         SERIAL     PRIMARY KEY,
    username   TEXT       NOT NULL,
    hash       TEXT       NOT NULL,
    created_at TIMESTAMP  NOT NULL
)
