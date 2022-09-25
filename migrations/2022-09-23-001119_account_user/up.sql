CREATE TABLE
    account_user (
        id BIGSERIAL PRIMARY KEY,
        account_user_uuid UUID NOT NULL,
        email VARCHAR(100) NOT NULL,
        hash VARCHAR NOT NULL,
        created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
        updated_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP
    );

CREATE UNIQUE INDEX account_user_account_user_uuid_key ON account_user(account_user_uuid);

CREATE UNIQUE INDEX account_user_email_key ON account_user(email);

CREATE INDEX ix_account_user_email ON account_user(email);

-- Your SQL goes here