extern crate actix;
#[macro_use]
extern crate diesel;
#[macro_use]
extern crate diesel_migrations;

extern crate dotenv;

mod actors;
pub mod app_config;
mod db_utils;
pub mod errors;
mod models;
pub mod routes;
mod schema;
pub mod utils;

use actix_web::{web::Data, App, HttpServer};

use actix::SyncArbiter;
use actors::DbActor;
use app_config::config_app;
use db_utils::{get_pool, run_migrations};
use dotenv::dotenv;
use models::AppState;
use std::env;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    let db_url = env::var("DATABASE_URL").expect("Error retrieving the database url");
    run_migrations(&db_url);
    let pool = get_pool(&db_url);
    let db_addr = SyncArbiter::start(5, move || DbActor(pool.clone()));

    HttpServer::new(move || {
        App::new()
            .configure(config_app)
            .app_data(Data::new(AppState {
                db: db_addr.clone(),
            }))
    })
    .bind(("127.0.0.1", 4000))?
    .run()
    .await
}
