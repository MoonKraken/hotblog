--- Add up migration script here
CREATE TABLE post (
    id VARCHAR NOT NULL PRIMARY KEY,
    dt VARCHAR NOT NULL,
    image_url VARCHAR,
    title VARCHAR NOT NULL,
    text VARCHAR NOT NULL
);
