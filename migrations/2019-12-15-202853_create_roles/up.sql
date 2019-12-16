CREATE TABLE roles (
    id       SERIAL  PRIMARY KEY,
    name     TEXT    NOT NULL,
    admin    BOOLEAN NOT NULL,
    download BOOLEAN NOT NULL,
    edit     BOOLEAN NOT NULL,
    upload   BOOLEAN NOT NULL,
    view     BOOLEAN NOT NULL
)
