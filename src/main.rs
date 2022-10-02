#[macro_use]
extern crate diesel;

use actix_web::{middleware, web, App, HttpServer};
use diesel::prelude::*;
use diesel::r2d2::{self, ConnectionManager};

mod actions;
mod models;
mod schema;
mod handlers;
mod utils;

use crate::handlers::collections;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv::dotenv().ok();
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));

    let hostname = std::env::var("HOST").expect("HOST");
    let port = std::env::var("PORT").expect("PORT").parse::<u16>().unwrap();

    // set up database connection pool
    let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL");
    let manager = ConnectionManager::<SqliteConnection>::new(database_url);
    let pool = r2d2::Pool::builder()
        .build(manager)
        .expect("Failed to create pool.");

    log::info!("starting HTTP server at http://{}:{}", hostname, port);

    // Start HTTP server
    HttpServer::new(move || {
        App::new()
            // set up DB pool to be used with web::Data<Pool> extractor
            .app_data(web::Data::new(pool.clone()))
            .wrap(middleware::Logger::default())
            .service(collections::get_collection)
            .service(collections::get_collections)
            .service(collections::insert_collection)
            .service(collections::delete_collection)
    })
        .bind((hostname, port))?
        .run()
        .await
}