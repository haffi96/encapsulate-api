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

use actix_web::{web::Data, App, HttpServer, middleware};

use actix::SyncArbiter;
use actors::DbActor;
use app_config::config_app;
use db_utils::{get_pool, run_migrations};
use dotenv::dotenv;
use models::AppState;
use std::env;
use actix_session::{storage::RedisActorSessionStore, SessionMiddleware};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));
    log::info!("starting HTTP server at http://localhost:4000");

    dotenv().ok();
    let db_url = env::var("DATABASE_URL").expect("Error retrieving the database url");
    let redis = env::var("REDIS_URL").expect("Error retrieving the database url");
    println!("redis url: {}", &redis);

    // FIXME: Use key from env
    let private_key = actix_web::cookie::Key::generate();

    run_migrations(&db_url);
    let pool = get_pool(&db_url);
    let db_addr = SyncArbiter::start(5, move || DbActor(pool.clone()));

    HttpServer::new(move || {
        App::new()
            .wrap(
                SessionMiddleware::builder(
                    RedisActorSessionStore::new(&redis),
                    private_key.clone(),
                )
                .build(),
            )
            // enable logger - always register Actix Web Logger middleware last
            .wrap(middleware::Logger::default())
            .configure(config_app)
            .app_data(Data::new(AppState {
                db: db_addr.clone(),
            }))
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
