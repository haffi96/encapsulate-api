use crate::actors::users::{CreateUser, DeleteUser, GetUser, UpdateUser};
use crate::models::{AccountUserData, AppState, UpdateAccountUserData};

use actix_web::{
    delete, get, post, put,
    web::{Data, Json, Path},
    HttpResponse, Responder,
};

#[post("/register")]
pub async fn create_user(
    account_user: Json<AccountUserData>,
    state: Data<AppState>,
) -> impl Responder {
    let db = state.as_ref().db.clone();
    let account_user = account_user.into_inner();

    match db
        .send(CreateUser {
            email: account_user.email,
        })
        .await
    {
        Ok(Ok(account_user)) => HttpResponse::Ok().json(account_user),
        _ => HttpResponse::InternalServerError().json("Something went wrong"),
    }
}

#[delete("/{account_id}")]
async fn delete_account(path: Path<i64>, state: Data<AppState>) -> impl Responder {
    let db = state.as_ref().db.clone();
    let account_id = path.into_inner();

    match db
        .send(DeleteUser {
            account_user_id: account_id,
        })
        .await
    {
        Ok(Ok(account_user)) => HttpResponse::Ok().json(account_user),
        Ok(Err(_)) => HttpResponse::NotFound().json("Account not found"),
        _ => HttpResponse::InternalServerError().json("Something went wrong"),
    }
}

#[put("/{account_id}")]
async fn update_user(
    path: Path<i64>,
    account_data: Json<UpdateAccountUserData>,
    state: Data<AppState>,
) -> impl Responder {
    let db = state.as_ref().db.clone();
    let account_id = path.into_inner();
    let account_data = account_data.into_inner();

    match db
        .send(UpdateUser {
            account_user_id: account_id,
            email: account_data.email,
        })
        .await
    {
        Ok(Ok(account_user)) => HttpResponse::Ok().json(account_user),
        Ok(Err(_)) => HttpResponse::NotFound().json("Account not found"),
        _ => HttpResponse::InternalServerError().json("Something went wrong"),
    }
}

#[get("/{account_id}")]
async fn get_account_holder(path: Path<i64>, state: Data<AppState>) -> impl Responder {
    let db = state.as_ref().db.clone();
    let account_id = path.into_inner();

    match db
        .send(GetUser {
            account_user_id: account_id,
        })
        .await
    {
        Ok(Ok(account_user)) => HttpResponse::Ok().json(account_user),
        _ => HttpResponse::InternalServerError().json("Something went wrong"),
    }
}
