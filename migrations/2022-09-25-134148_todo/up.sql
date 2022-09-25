-- Your SQL goes here

CREATE TABLE
    todo (
        id BIGSERIAL PRIMARY KEY,
        account_user_id bigint NOT NULL,
        todo_uuid UUID NOT NULL,
        body TEXT NOT NULL,
        completed BOOLEAN NOT NULL DEFAULT FALSE,
        reminder_time TIMESTAMP,
        created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
        updated_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
        CONSTRAINT account_user_id_fkey FOREIGN key(account_user_id) REFERENCES account_user(id)
    );

CREATE UNIQUE INDEX todo_uuid_unq ON todo(todo_uuid);

CREATE INDEX ix_todo_account_user_id ON todo(account_user_id);