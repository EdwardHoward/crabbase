use actix_web::{get, web, Error, HttpResponse};
use diesel::prelude::*;
use diesel::r2d2::{self, ConnectionManager};
use crate::actions::collections;

type DbPool = r2d2::Pool<ConnectionManager<SqliteConnection>>;

#[get("/api/collections")]
pub async fn get_collections(pool: web::Data<DbPool>) -> Result<HttpResponse, Error> {
    let collections = web::block(move || {
        let mut conn = pool.get()?;
        collections::get_collections(&mut conn)
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

#[get("/api/collections/{id}")]
pub async fn get_collections_by_id(pool: web::Data<DbPool>, id: web::Path<String>) -> Result<HttpResponse, Error> {
    let id_str = id.into_inner();

    let collection = web::block(move || {
      let mut conn = pool.get()?;
      collections::get_collection_by_id(&mut conn, &id_str)
    })
    .await?
    .map_err(actix_web::error::ErrorInternalServerError)?;

    Ok(HttpResponse::Ok().json(collection))
}
