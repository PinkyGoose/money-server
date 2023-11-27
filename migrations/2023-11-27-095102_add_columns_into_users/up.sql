-- Your SQL goes here
ALTER TABLE users
    ADD
 enc_pass VARCHAR NOT NULL;
 ALTER TABLE users
    ADD
 token VARCHAR;

