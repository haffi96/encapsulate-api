extern crate actix;

extern crate dotenv;

use actix_web::{
    delete, get, post, put,
    web::{Data, Json, Path},
    HttpResponse, Responder,
};

use crate::actors::todos::{CreateTodo, DeleteTodo, GetTodos, UpdateTodo};
use crate::models::{AppState, NewTodoData, UpdateTodoData};
use uuid::Uuid;

#[post("/new_todo")]
async fn create_todo(todo: Json<NewTodoData>, state: Data<AppState>) -> impl Responder {
    let db = state.as_ref().db.clone();
    let todo = todo.into_inner();

    match db
        .send(CreateTodo {
            account_user_id: todo.account_user_id,
            todo_uuid: Uuid::new_v4(),
            body: todo.body,
        })
        .await
    {
        Ok(Ok(todo)) => HttpResponse::Ok().json(todo),
        _ => HttpResponse::InternalServerError().json("Something went wrong"),
    }
}

#[delete("/{todo_uuid}")]
async fn delete_todo(path: Path<Uuid>, state: Data<AppState>) -> impl Responder {
    let db = state.as_ref().db.clone();
    let todo_uuid = path.into_inner();

    match db
        .send(DeleteTodo {
            todo_uuid: todo_uuid,
        })
        .await
    {
        Ok(Ok(todo)) => HttpResponse::Ok().json(todo),
        Ok(Err(_)) => HttpResponse::NotFound().json("Todo not found"),
        _ => HttpResponse::InternalServerError().json("Something went wrong"),
    }
}

#[put("/{todo_uuid}")]
async fn update_todo(
    path: Path<Uuid>,
    todo: Json<UpdateTodoData>,
    state: Data<AppState>,
) -> impl Responder {
    let db = state.as_ref().db.clone();
    let todo_uuid = path.into_inner();
    let todo = todo.into_inner();

    match db
        .send(UpdateTodo {
            todo_uuid: todo_uuid,
            body: todo.body,
            completed: todo.completed,
            reminder_time: todo.reminder_time,
            updated_at: todo.updated_at,
        })
        .await
    {
        Ok(Ok(todo)) => HttpResponse::Ok().json(todo),
        Ok(Err(_)) => HttpResponse::NotFound().json("Todo not found"),
        _ => HttpResponse::InternalServerError().json("Something went wrong"),
    }
}

#[get("/{account_user_id}")]
async fn get_todos_for_account_user(path: Path<i64>, state: Data<AppState>) -> impl Responder {
    let db = state.as_ref().db.clone();
    let account_user_id = path.into_inner();

    match db
        .send(GetTodos {
            account_user_id: account_user_id,
        })
        .await
    {
        Ok(Ok(todos)) => HttpResponse::Ok().json(todos),
        _ => HttpResponse::InternalServerError().json("Something went wrong"),
    }
}
