use actix_web::web;

use crate::routes::notes::{create_note, delete_note, get_notes_for_account_user, update_note};
use crate::routes::users::{create_user, delete_account, get_account_holder, update_user};

pub fn config_app(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/accounts")
            .service(create_user)
            .service(update_user)
            .service(delete_account)
            .service(get_account_holder),
    )
    .service(
        web::scope("/notes")
            .service(create_note)
            .service(update_note)
            .service(delete_note)
            .service(get_notes_for_account_user),
    );
}
