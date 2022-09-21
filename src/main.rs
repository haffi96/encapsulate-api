extern crate actix;
#[macro_use]
extern crate diesel;
#[macro_use]
extern crate diesel_migrations;

extern crate dotenv;

mod models;
mod db_utils;
mod schema;
mod actors;

use actix_web::{App, HttpResponse, HttpServer, Responder, delete, get, post, put, web::{Data, Json, Path}};

use actix::SyncArbiter;
use actors::notes::{Create, DbActor, Delete, GetNotes, Update};
use db_utils::{get_pool, run_migrations};
use models::{AppState, NoteData};
use dotenv::dotenv;
use std::env;
use uuid::Uuid;


#[post("/note")]
async fn create_note(note: Json<NoteData>, state: Data<AppState>) -> impl Responder {
    let db = state.as_ref().db.clone();
    let note = note.into_inner();

    match db.send(Create { title: note.title, body: note.body }).await {
        Ok(Ok(note)) => HttpResponse::Ok().json(note),
        _ => HttpResponse::InternalServerError().json("Something went wrong")
    }
}

#[delete("/{id}")]
async fn delete_note(path: Path<Uuid>, state: Data<AppState>) -> impl Responder {
    let db = state.as_ref().db.clone();
    let note_id = path.into_inner();

    match db.send(Delete { id: note_id }).await {
        Ok(Ok(note)) => HttpResponse::Ok().json(note),
        Ok(Err(_)) => HttpResponse::NotFound().json("Article not found"),
        _ => HttpResponse::InternalServerError().json("Something went wrong")
    }
}

#[put("/{id}")]
async fn update_note(path: Path<Uuid>, note: Json<NoteData>, state: Data<AppState>) -> impl Responder {
    let db = state.as_ref().db.clone();
    let note_id = path.into_inner();
    let note = note.into_inner();
    let now = chrono::offset::Utc::now().timestamp();
    let naive = chrono::NaiveDateTime::from_timestamp(now, 0);

    match db.send(Update { id: note_id, title: note.title, body: note.body, updated_at: naive }).await {
        Ok(Ok(note)) => HttpResponse::Ok().json(note),
        Ok(Err(_)) => HttpResponse::NotFound().json("Note not found"),
        _ => HttpResponse::InternalServerError().json("Something went wrong")
    }
}

#[get("/notes")]
async fn get_notes(state: Data<AppState>) -> impl Responder {
    let db = state.as_ref().db.clone();

    match db.send(GetNotes).await {
        Ok(Ok(notes)) => HttpResponse::Ok().json(notes),
        _ => HttpResponse::InternalServerError().json("Something went wrong")
    }
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // enable logs
    std::env::set_var("RUST_LOG", "debug");
    std::env::set_var("RUST_BACKTRACE", "1");
    env_logger::init();

    dotenv().ok();
    let db_url = env::var("DATABASE_URL").expect("Error retrieving the database url");
    run_migrations(&db_url);
    let pool = get_pool(&db_url);
    let db_addr = SyncArbiter::start(5, move || DbActor(pool.clone()));

    HttpServer::new(move || {
        App::new()
            .service(get_notes)
            .service(delete_note)
            .service(create_note)
            .service(update_note)
            .app_data(Data::new(AppState {
                db: db_addr.clone()
            }))
    })
    .bind(("0.0.0.0", 4000))?
    .run()
    .await
}