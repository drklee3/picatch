CREATE TABLE sessions (
    id            TEXT      PRIMARY KEY,
    user_id       INTEGER   NOT NULL,
    ip_address    INET,
    user_agent    TEXT,
    last_activity TIMESTAMP NOT NULL,
    FOREIGN KEY (user_id) REFERENCES users (id)
)
