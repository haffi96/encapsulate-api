use crate::actix::{Handler, Message};
use crate::actors::DbActor;
use crate::diesel::prelude::*;
use crate::diesel::result::Error;
use crate::models::{AccountUser, NewAccountUser};
use crate::schema::account_user::dsl::{account_user, email, id};
use crate::utils::{hash_password, verify};
use uuid::Uuid;

#[derive(Message)]
#[rtype(result = "QueryResult<AccountUser>")]
pub struct CreateUser {
    pub email: String,
    pub pwd: String,
}

#[derive(Message)]
#[rtype(result = "QueryResult<AccountUser>")]
pub struct UpdateUser {
    pub account_user_id: i64,
    pub email: String,
}

#[derive(Message)]
#[rtype(result = "QueryResult<AccountUser>")]
pub struct DeleteUser {
    pub account_user_id: i64,
}

#[derive(Message)]
#[rtype(result = "QueryResult<AccountUser>")]
pub struct GetUser {
    pub account_user_id: i64,
}

#[derive(Message)]
#[rtype(result = "QueryResult<AccountUser>")]
pub struct AuthUser {
    pub email: String,
    pub pwd: String,
}

impl Handler<CreateUser> for DbActor {
    type Result = QueryResult<AccountUser>;

    fn handle(&mut self, msg: CreateUser, _: &mut Self::Context) -> Self::Result {
        let conn = self.0.get().expect("Unable to get a connection");

        let hashed_pwd: String = hash_password(&msg.pwd).expect("Failed to hash password");
        let new_user = NewAccountUser {
            account_user_uuid: Uuid::new_v4(),
            hash: hashed_pwd,
            email: msg.email,
        };

        diesel::insert_into(account_user)
            .values(new_user)
            .get_result::<AccountUser>(&conn)
    }
}

impl Handler<UpdateUser> for DbActor {
    type Result = QueryResult<AccountUser>;

    fn handle(&mut self, msg: UpdateUser, _: &mut Self::Context) -> Self::Result {
        let conn = self.0.get().expect("Unable to get a connection");

        diesel::update(account_user)
            .filter(id.eq(msg.account_user_id))
            .set((email.eq(msg.email),))
            .get_result::<AccountUser>(&conn)
    }
}

impl Handler<DeleteUser> for DbActor {
    type Result = QueryResult<AccountUser>;

    fn handle(&mut self, msg: DeleteUser, _: &mut Self::Context) -> Self::Result {
        let conn = self.0.get().expect("Unable to get a connection");

        diesel::delete(account_user)
            .filter(id.eq(msg.account_user_id))
            .get_result::<AccountUser>(&conn)
    }
}

impl Handler<GetUser> for DbActor {
    type Result = QueryResult<AccountUser>;

    fn handle(&mut self, msg: GetUser, _: &mut Self::Context) -> Self::Result {
        let conn = self.0.get().expect("Unable to get a connection");
        account_user
            .filter(id.eq(msg.account_user_id))
            .get_result::<AccountUser>(&conn)
    }
}

impl Handler<AuthUser> for DbActor {
    type Result = QueryResult<AccountUser>;

    fn handle(&mut self, msg: AuthUser, _: &mut Self::Context) -> Self::Result {
        let conn = self.0.get().expect("Unable to get a connection");

        let user = account_user
            .filter(email.eq(msg.email))
            .first::<AccountUser>(&conn)
            .expect("Account not found");

        if let Ok(matching) = verify(&user.hash, &msg.pwd) {
            if matching {
                return Ok(user);
            }
        };
        Err(Error::NotFound)
    }
}