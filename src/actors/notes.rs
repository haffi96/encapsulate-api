
use crate::actix::{Actor, Handler, Message, SyncContext};
use crate::diesel::prelude::*;
use crate::models::{Note, NewNote};
use crate::schema::note::dsl::{note, id, title, body, updated_at};
use chrono::NaiveDateTime;
use diesel::{
    r2d2::{ConnectionManager, Pool},
    PgConnection,
};
use uuid::Uuid;

pub struct DbActor(pub Pool<ConnectionManager<PgConnection>>);


#[derive(Message)]
#[rtype(result="QueryResult<Note>")]
pub struct Create {
    pub title: String,
    pub body: String,
}

#[derive(Message)]
#[rtype(result="QueryResult<Note>")]
pub struct Update {
    pub id: Uuid,
    pub title: String,
    pub body: String,
    pub updated_at: NaiveDateTime
}

#[derive(Message)]
#[rtype(result="QueryResult<Note>")]
pub struct Delete {
    pub id: Uuid
}

#[derive(Message)]
#[rtype(result="QueryResult<Vec<Note>>")]
pub struct GetNotes;


impl Actor for DbActor {
    type Context = SyncContext<Self>;
}

impl Handler<Create> for DbActor {
    type Result = QueryResult<Note>;

    fn handle(&mut self, msg: Create, _: &mut Self::Context) -> Self::Result {
        let conn = self.0.get().expect("Unable to get a connection");
        let new_note = NewNote {
            id: 1,
            account_user_id: 1,
            note_uuid: Uuid::new_v4(),
            title: msg.title,
            body: msg.body,
        };

        diesel::insert_into(note)
        .values(new_note)
        .get_result::<Note>(&conn)
    }
}

impl Handler<Update> for DbActor {
    type Result = QueryResult<Note>;

    fn handle(&mut self, msg: Update, _: &mut Self::Context) -> Self::Result {
        let conn = self.0.get().expect("Unable to get a connection");

        diesel::update(note)
        .filter(id.eq(msg.id))
        .set((title.eq(msg.title), body.eq(msg.body), updated_at.eq(msg.updated_at)))
        .get_result::<Note>(&conn)
    }
}

impl Handler<Delete> for DbActor {
    type Result = QueryResult<Note>;

    fn handle(&mut self, msg: Delete, _: &mut Self::Context) -> Self::Result {
        let conn = self.0.get().expect("Unable to get a connection");

        diesel::delete(note)
                .filter(id.eq(msg.id))
                .get_result::<Note>(&conn)
    }
}

impl Handler<GetNotes> for DbActor {
    type Result = QueryResult<Vec<Note>>;

    fn handle(&mut self, msg: GetNotes, _: &mut Self::Context) -> Self::Result {
        let conn = self.0.get().expect("Unable to get a connection");
        note.get_results::<Note>(&conn)
    }
}
