use crate::actors::users::{AuthUser, CreateUser, DeleteUser, GetUser, UpdateUser};
use crate::errors::ServiceError;
use crate::models::{AccountUserData, AppState, UpdateAccountUserData};
use actix_session::Session;

use actix_web::{
    delete, get, post, put,
    web::{Data, Json, Path},
    HttpResponse, Responder
};
use uuid::Uuid;

#[post("/register")]
pub async fn create_user(
    account_user: Json<AccountUserData>,
    state: Data<AppState>,
    session: Session,
) -> impl Responder {
    let db = state.as_ref().db.clone();
    let account_user = account_user.into_inner();

    match db
        .send(CreateUser {
            email: account_user.email,
            pwd: account_user.pwd,
        })
        .await
    {
        Ok(Ok(account_user)) => {
            session.insert(account_user.account_user_uuid.to_string(), "token").expect("Failed to save session");
            HttpResponse::Ok().json(account_user)
        },
        _ => HttpResponse::InternalServerError().json("Something went wrong"),
    }
}

#[post("/login")]
pub async fn login_user(
    account_user: Json<AccountUserData>,
    state: Data<AppState>,
    session: Session,
) -> impl Responder {
    let db = state.as_ref().db.clone();
    let account_user = account_user.into_inner();

    match db
        .send(AuthUser {
            email: account_user.email,
            pwd: account_user.pwd,
        })
        .await
    {
        Ok(Ok(account_user)) => {
            session.insert(account_user.account_user_uuid.to_string(), "token").expect("Failed to save session");
            session.renew();
            HttpResponse::Ok().json(account_user)
        },
        _ => HttpResponse::Unauthorized().json("Credentials incorrect"),
    }
}


pub fn validate_session(account_id: &i64, session: &Session) -> impl Responder {
    let user_id: Option<i64> = session.get(&account_id.to_string()).expect("Failed to find sessoin");

    match user_id {
        Some(_id) => {
            // keep the user's session alive
            session.renew();
            HttpResponse::Ok()
        }
        None => HttpResponse::Unauthorized(),
    }
}


#[post("/logout/{account_uuid}")]
pub async fn logout_user(
    path: Path<i64>,
    state: Data<AppState>,
    session: Session,
) -> impl Responder {
    
    let db = state.as_ref().db.clone();
    let account_id = path.into_inner();
    validate_session(&account_id, &session);

    match db
    .send(GetUser {
        account_user_id: account_id
    })
    .await
    {
        Ok(_) => {
            session.purge();
            HttpResponse::Ok().json(format!("Logged out user"))
        },
        _ => (HttpResponse::Unauthorized().json("Unable to logout user"))
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
