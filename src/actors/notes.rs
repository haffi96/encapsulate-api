use crate::actix::{Handler, Message};
use crate::actors::DbActor;
use crate::diesel::prelude::*;
use crate::models::{NewNote, Note};
use crate::schema::note::dsl::{account_user_id, body, note, note_uuid, title, updated_at};
use uuid::Uuid;

use std::time::SystemTime;

#[derive(Message)]
#[rtype(result = "QueryResult<Note>")]
pub struct CreateNote {
    pub account_user_id: i64,
    pub note_uuid: Uuid,
    pub title: String,
    pub body: String,
}

#[derive(Message)]
#[rtype(result = "QueryResult<Note>")]
pub struct UpdateNote {
    pub note_uuid: Uuid,
    pub title: String,
    pub body: String,
    pub updated_at: SystemTime,
}

#[derive(Message)]
#[rtype(result = "QueryResult<Note>")]
pub struct DeleteNote {
    pub note_uuid: Uuid,
}

#[derive(Message)]
#[rtype(result = "QueryResult<Vec<Note>>")]
pub struct GetNotes {
    pub account_user_id: i64,
}

impl Handler<CreateNote> for DbActor {
    type Result = QueryResult<Note>;

    fn handle(&mut self, msg: CreateNote, _: &mut Self::Context) -> Self::Result {
        let conn = self.0.get().expect("Unable to get a connection");
        let new_note = NewNote {
            account_user_id: msg.account_user_id,
            note_uuid: msg.note_uuid,
            title: msg.title,
            body: msg.body,
        };

        diesel::insert_into(note)
            .values(new_note)
            .get_result::<Note>(&conn)
    }
}

impl Handler<UpdateNote> for DbActor {
    type Result = QueryResult<Note>;

    fn handle(&mut self, msg: UpdateNote, _: &mut Self::Context) -> Self::Result {
        let conn = self.0.get().expect("Unable to get a connection");

        diesel::update(note)
            .filter(note_uuid.eq(msg.note_uuid))
            .set((
                title.eq(msg.title),
                body.eq(msg.body),
                updated_at.eq(msg.updated_at),
            ))
            .get_result::<Note>(&conn)
    }
}

impl Handler<DeleteNote> for DbActor {
    type Result = QueryResult<Note>;

    fn handle(&mut self, msg: DeleteNote, _: &mut Self::Context) -> Self::Result {
        let conn = self.0.get().expect("Unable to get a connection");

        diesel::delete(note)
            .filter(note_uuid.eq(msg.note_uuid))
            .get_result::<Note>(&conn)
    }
}

impl Handler<GetNotes> for DbActor {
    type Result = QueryResult<Vec<Note>>;

    fn handle(&mut self, msg: GetNotes, _: &mut Self::Context) -> Self::Result {
        let conn = self.0.get().expect("Unable to get a connection");
        note.filter(account_user_id.eq(msg.account_user_id))
            .get_results::<Note>(&conn)
    }
}
