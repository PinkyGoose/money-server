-- This file should undo anything in `up.sql`
ALTER TABLE books
DROP COLUMN enc_pass;
ALTER TABLE books
DROP COLUMN token;