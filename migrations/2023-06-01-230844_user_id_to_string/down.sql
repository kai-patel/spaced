-- This file should undo anything in `up.sql`
ALTER TABLE users DROP COLUMN id;
ALTER TABLE users RENAME COLUMN idx TO id;
