-- Your SQL goes here
CREATE TABLE users (
    uuid VARCHAR(64) NOT NULL PRIMARY KEY,
    login VARCHAR NOT NULL,
    hash VARCHAR(64) NOT NULL
)