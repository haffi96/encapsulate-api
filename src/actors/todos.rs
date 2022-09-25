use crate::actix::{Handler, Message};
use crate::actors::DbActor;
use crate::diesel::prelude::*;
use crate::models::{NewTodo, Todo};
use crate::schema::todo::dsl::{
    account_user_id, body, completed, reminder_time, todo, todo_uuid, updated_at,
};
use chrono::NaiveDateTime;
use uuid::Uuid;

#[derive(Message)]
#[rtype(result = "QueryResult<Todo>")]
pub struct CreateTodo {
    pub account_user_id: i64,
    pub todo_uuid: Uuid,
    pub body: String,
}

#[derive(Message)]
#[rtype(result = "QueryResult<Todo>")]
pub struct UpdateTodo {
    pub todo_uuid: Uuid,
    pub body: String,
    pub completed: bool,
    pub reminder_time: Option<NaiveDateTime>,
    pub updated_at: NaiveDateTime,
}

#[derive(Message)]
#[rtype(result = "QueryResult<Todo>")]
pub struct DeleteTodo {
    pub todo_uuid: Uuid,
}

#[derive(Message)]
#[rtype(result = "QueryResult<Vec<Todo>>")]
pub struct GetTodos {
    pub account_user_id: i64,
}

impl Handler<CreateTodo> for DbActor {
    type Result = QueryResult<Todo>;

    fn handle(&mut self, msg: CreateTodo, _: &mut Self::Context) -> Self::Result {
        let conn = self.0.get().expect("Unable to get a connection");
        let new_todo = NewTodo {
            account_user_id: msg.account_user_id,
            todo_uuid: msg.todo_uuid,
            body: msg.body,
            completed: false,
        };

        diesel::insert_into(todo)
            .values(new_todo)
            .get_result::<Todo>(&conn)
    }
}

impl Handler<UpdateTodo> for DbActor {
    type Result = QueryResult<Todo>;

    fn handle(&mut self, msg: UpdateTodo, _: &mut Self::Context) -> Self::Result {
        let conn = self.0.get().expect("Unable to get a connection");

        diesel::update(todo)
            .filter(todo_uuid.eq(msg.todo_uuid))
            .set((
                body.eq(msg.body),
                updated_at.eq(msg.updated_at),
                completed.eq(msg.completed),
                reminder_time.eq(msg.reminder_time),
            ))
            .get_result::<Todo>(&conn)
    }
}

impl Handler<DeleteTodo> for DbActor {
    type Result = QueryResult<Todo>;

    fn handle(&mut self, msg: DeleteTodo, _: &mut Self::Context) -> Self::Result {
        let conn = self.0.get().expect("Unable to get a connection");

        diesel::delete(todo)
            .filter(todo_uuid.eq(msg.todo_uuid))
            .get_result::<Todo>(&conn)
    }
}

impl Handler<GetTodos> for DbActor {
    type Result = QueryResult<Vec<Todo>>;

    fn handle(&mut self, msg: GetTodos, _: &mut Self::Context) -> Self::Result {
        let conn = self.0.get().expect("Unable to get a connection");
        todo.filter(account_user_id.eq(msg.account_user_id))
            .get_results::<Todo>(&conn)
    }
}
