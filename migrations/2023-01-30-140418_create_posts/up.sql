-- Your SQL goes here
CREATE TABLE posts (
    uuid VARCHAR(64) NOT NULL PRIMARY KEY,
    author_uuid VARCHAR(64) NOT NULL,
    title VARCHAR(60) NOT NULL,
    content TEXT NOT NULL,
    FOREIGN KEY (author_uuid) REFERENCES users(uuid)
)