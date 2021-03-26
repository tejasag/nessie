CREATE TABLE users
(
    id           serial    not null primary key unique,
    username     VARCHAR   NOT NULL UNIQUE,
    display_name VARCHAR   NOT NULL,
    email        VARCHAR   NOT NULL,
    password     VARCHAR   NOT NULL,
    created_at   VARCHAR NOT NULL,
    last_updated VARCHAR
)
