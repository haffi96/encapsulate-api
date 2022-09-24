// models.rs
use super::schema::*;
use crate::actors::DbActor;
use actix::Addr;
use chrono::NaiveDateTime;
use diesel::sql_types::Timestamp;
use serde::{Deserialize, Serialize};
use std::time::SystemTime;
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
    pub created_at: SystemTime,
    pub updated_at: SystemTime,
}

#[derive(Debug, Serialize, Deserialize, Associations, Queryable, Insertable)]
#[table_name = "account_user"]
pub struct NewAccountUser {
    pub account_user_uuid: Uuid,
    pub email: String,
}

#[derive(Serialize, Deserialize)]
pub struct AccountUserData {
    pub email: String,
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
    pub created_at: SystemTime,
    pub updated_at: SystemTime,
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
    pub update_at: SystemTime,
}
