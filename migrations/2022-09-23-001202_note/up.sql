-- Your SQL goes here
CREATE TABLE note (
    id BIGSERIAL PRIMARY KEY,
    account_user_id bigint,
    note_uuid UUID NOT NULL,
    title VARCHAR NOT NULL,
    body TEXT NOT NULL,
    created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    CONSTRAINT account_user_id_fkey FOREIGN key(account_user_id) REFERENCES account_user(id)
);

CREATE UNIQUE INDEX note_uuid_unq ON note(note_uuid);
CREATE INDEX ix_note_account_user_id ON note(account_user_id);