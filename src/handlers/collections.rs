use actix_web::{get, middleware, web, App, Error, HttpResponse, HttpServer};
use diesel::prelude::*;
use diesel::r2d2::{self, ConnectionManager};
use crate::actions;

type DbPool = r2d2::Pool<ConnectionManager<SqliteConnection>>;

#[get("/collections")]
pub async fn get_collections(pool: web::Data<DbPool>) -> Result<HttpResponse, Error> {
    let collections = web::block(move || {
        let mut conn = pool.get()?;
        actions::get_collections(&mut conn)
    })
    .await?
    .map_err(actix_web::error::ErrorInternalServerError)?;

    if collections.len() > 0 {
        Ok(HttpResponse::Ok().json(collections))
    } else {
        let res = HttpResponse::NotFound().body(format!("No collections found"));
        Ok(res)
    }
}