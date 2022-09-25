-- This file should undo anything in `up.sql`

DROP INDEX todo_uuid_unq;

DROP INDEX ix_todo_account_user_id;

ALTER TABLE todo DROP CONSTRAINT account_user_id_fkey;

DROP TABLE todo;