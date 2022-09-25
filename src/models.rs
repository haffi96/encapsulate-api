// models.rs
use super::schema::*;
use crate::actors::DbActor;
use actix::Addr;
use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

pub struct AppState {
    pub db: Addr<DbActor>,
}

// account_user
#[derive(Identifiable, Debug, Serialize, Deserialize, Queryable, Insertable)]
#[table_name = "account_user"]
pub struct AccountUser {
    pub id: i64,
    pub account_user_uuid: Uuid,
    pub email: String,
    #[serde(skip_serializing)]
    pub hash: String,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(Debug, Serialize, Deserialize, Associations, Queryable, Insertable)]
#[table_name = "account_user"]
pub struct NewAccountUser {
    pub account_user_uuid: Uuid,
    pub hash: String,
    pub email: String,
}

#[derive(Serialize, Deserialize)]
pub struct AccountUserData {
    pub email: String,
    pub pwd: String,
}

#[derive(Serialize, Deserialize)]
pub struct UpdateAccountUserData {
    pub email: String,
}

// note
#[derive(Identifiable, Debug, Serialize, Deserialize, Associations, Queryable, Insertable)]
#[table_name = "note"]
pub struct Note {
    pub id: i64,
    pub account_user_id: i64,
    pub note_uuid: Uuid,
    pub title: String,
    pub body: String,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(Debug, Serialize, Deserialize, Associations, Queryable, Insertable)]
#[table_name = "note"]
pub struct NewNote {
    pub account_user_id: i64,
    pub note_uuid: Uuid,
    pub title: String,
    pub body: String,
}

#[derive(Serialize, Deserialize)]
pub struct NewNoteData {
    pub account_user_id: i64,
    pub title: String,
    pub body: String,
}

#[derive(Serialize, Deserialize)]
pub struct UpdateNoteData {
    pub title: String,
    pub body: String,
    pub updated_at: NaiveDateTime,
}

// todo
#[derive(Identifiable, Debug, Serialize, Deserialize, Associations, Queryable, Insertable)]
#[table_name = "todo"]
pub struct Todo {
    pub id: i64,
    pub account_user_id: i64,
    pub todo_uuid: Uuid,
    pub body: String,
    pub completed: bool,
    pub reminder_time: Option<NaiveDateTime>,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(Debug, Serialize, Deserialize, Associations, Queryable, Insertable)]
#[table_name = "todo"]
pub struct NewTodo {
    pub account_user_id: i64,
    pub todo_uuid: Uuid,
    pub body: String,
    pub completed: bool,
}

#[derive(Serialize, Deserialize)]
pub struct NewTodoData {
    pub account_user_id: i64,
    pub body: String,
}

#[derive(Serialize, Deserialize)]
pub struct UpdateTodoData {
    pub body: String,
    pub completed: bool,
    pub reminder_time: Option<NaiveDateTime>,
    pub updated_at: NaiveDateTime,
}
