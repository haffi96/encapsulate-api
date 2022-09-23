// models.rs
use super::schema::*;
use serde::{Deserialize, Serialize};
use crate::actors::notes::DbActor;
use chrono::NaiveDateTime;

use uuid::Uuid;
use actix::Addr;


pub struct AppState {
    pub db: Addr<DbActor>
 }


// account_user
#[derive(Identifiable, Debug, Serialize, Deserialize, Queryable, Insertable)]
#[table_name = "account_user"]
pub struct AccountUser {
    pub id: i64,
    pub account_user_uuid: Uuid,
    pub email: String,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
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


#[derive(Identifiable, Debug, Serialize, Deserialize, Associations, Queryable, Insertable)]
#[table_name = "note"]
pub struct NewNote {
    pub id: Uuid,
    pub account_user_id: i64,
    pub note_uuid: Uuid,
    pub title: String,
    pub body: String,
}

#[derive(Serialize, Deserialize)]
pub struct NoteData {
    pub title: String,
    pub body: String
}