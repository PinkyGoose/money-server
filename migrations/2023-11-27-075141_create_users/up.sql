-- Your SQL goes here
CREATE TABLE users (
 id SERIAL PRIMARY KEY,
 username VARCHAR NOT NULL,
 email VARCHAR,
 registered TIMESTAMP NOT NULL DEFAULT 'now()'
);