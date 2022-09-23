// @generated automatically by Diesel CLI.

diesel::table! {
    account_user (id) {
        id -> Int8,
        account_user_uuid -> Uuid,
        email -> Varchar,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

diesel::table! {
    note (id) {
        id -> Int8,
        account_user_id -> Nullable<Int8>,
        note_uuid -> Uuid,
        title -> Varchar,
        body -> Text,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

diesel::joinable!(note -> account_user (account_user_id));

diesel::allow_tables_to_appear_in_same_query!(
    account_user,
    note,
);
