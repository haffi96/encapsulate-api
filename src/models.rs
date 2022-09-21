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


#[derive(Debug, Serialize, Deserialize, Queryable, Insertable)]
pub struct Note {
    pub id: Uuid,
    pub title: String,
    pub body: String,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}


#[derive(Debug, Serialize, Deserialize, Queryable, Insertable)]
#[table_name = "notes"]
pub struct NewNote {
    pub id: Uuid,
    pub title: String,
    pub body: String,
}

#[derive(Serialize, Deserialize)]
pub struct NoteData {
    pub title: String,
    pub body: String
}