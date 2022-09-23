-- This file should undo anything in `up.sql`
DROP INDEX note_uuid_unq;
DROP INDEX ix_note_account_user_id;
ALTER TABLE note DROP CONSTRAINT account_user_id_fkey;
DROP TABLE note;