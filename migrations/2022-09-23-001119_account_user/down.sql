-- This file should undo anything in `up.sql`
DROP INDEX account_user_account_user_uuid_key;
DROP INDEX ix_account_user_email;
DROP TABLE account_user;