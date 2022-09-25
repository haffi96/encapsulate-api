// @generated automatically by Diesel CLI.

diesel::table! {
    account_user (id) {
        id -> Int8,
        account_user_uuid -> Uuid,
        email -> Varchar,
        hash -> Varchar,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

diesel::table! {
    note (id) {
        id -> Int8,
        account_user_id -> Int8,
        note_uuid -> Uuid,
        title -> Varchar,
        body -> Text,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

diesel::table! {
    todo (id) {
        id -> Int8,
        account_user_id -> Int8,
        todo_uuid -> Uuid,
        body -> Text,
        completed -> Bool,
        reminder_time -> Nullable<Timestamp>,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

diesel::allow_tables_to_appear_in_same_query!(
    account_user,
    note,
    todo,
);
