extern crate actix;

extern crate dotenv;

use actix_web::{
    delete, get, post, put,
    web::{Data, Json, Path},
    HttpResponse, Responder,
};

use crate::actors::notes::{CreateNote, DeleteNote, GetNotes, UpdateNote};
use crate::models::{AppState, NewNoteData, UpdateNoteData};
use uuid::Uuid;

#[post("/new_note")]
async fn create_note(note: Json<NewNoteData>, state: Data<AppState>) -> impl Responder {
    let db = state.as_ref().db.clone();
    let note = note.into_inner();

    match db
        .send(CreateNote {
            account_user_id: note.account_user_id,
            note_uuid: Uuid::new_v4(),
            title: note.title,
            body: note.body,
        })
        .await
    {
        Ok(Ok(note)) => HttpResponse::Ok().json(note),
        _ => HttpResponse::InternalServerError().json("Something went wrong"),
    }
}

#[delete("/{note_uuid}")]
async fn delete_note(path: Path<Uuid>, state: Data<AppState>) -> impl Responder {
    let db = state.as_ref().db.clone();
    let note_uuid = path.into_inner();

    match db
        .send(DeleteNote {
            note_uuid: note_uuid,
        })
        .await
    {
        Ok(Ok(note)) => HttpResponse::Ok().json(note),
        Ok(Err(_)) => HttpResponse::NotFound().json("Article not found"),
        _ => HttpResponse::InternalServerError().json("Something went wrong"),
    }
}

#[put("/{note_uuid}")]
async fn update_note(
    path: Path<Uuid>,
    note: Json<UpdateNoteData>,
    state: Data<AppState>,
) -> impl Responder {
    let db = state.as_ref().db.clone();
    let note_uuid = path.into_inner();
    let note = note.into_inner();

    match db
        .send(UpdateNote {
            note_uuid: note_uuid,
            title: note.title,
            body: note.body,
            updated_at: note.update_at,
        })
        .await
    {
        Ok(Ok(note)) => HttpResponse::Ok().json(note),
        Ok(Err(_)) => HttpResponse::NotFound().json("Note not found"),
        _ => HttpResponse::InternalServerError().json("Something went wrong"),
    }
}

#[get("/{account_user_id}")]
async fn get_notes_for_account_user(path: Path<i64>, state: Data<AppState>) -> impl Responder {
    let db = state.as_ref().db.clone();
    let account_user_id = path.into_inner();

    match db
        .send(GetNotes {
            account_user_id: account_user_id,
        })
        .await
    {
        Ok(Ok(notes)) => HttpResponse::Ok().json(notes),
        _ => HttpResponse::InternalServerError().json("Something went wrong"),
    }
}
