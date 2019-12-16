CREATE TABLE sessions (
    id            TEXT      PRIMARY KEY,
    user_id       INTEGER   NOT NULL,
    ip_address    INET      NOT NULL,
    user_agent    TEXT      NOT NULL,
    last_activity TIMESTAMP NOT NULL,
    FOREIGN KEY (user_id) REFERENCES users (id)
)
