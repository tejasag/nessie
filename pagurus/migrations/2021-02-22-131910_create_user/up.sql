CREATE TABLE users
(
    username     VARCHAR NOT NULL PRIMARY KEY UNIQUE,
    display_name VARCHAR NOT NULL,
    email        VARCHAR NOT NULL,
    password     VARCHAR NOT NULL,
    created_at TIMESTAMP NOT NULL,
    last_updated TIMESTAMP
)