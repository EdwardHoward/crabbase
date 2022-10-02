use actix_web::{delete, get, post, web, Error, HttpResponse};
use diesel::prelude::*;
use diesel::r2d2::{self, ConnectionManager};
use crate::actions::collections;
use crate::models::collection::NewCollection;

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
pub async fn get_collection(pool: web::Data<DbPool>, id: web::Path<String>) -> Result<HttpResponse, Error> {
    let id_str = id.into_inner();

    let collection = web::block(move || {
      let mut conn = pool.get()?;
      collections::get_collection(&mut conn, &id_str)
    })
    .await?
    .map_err(actix_web::error::ErrorInternalServerError)?;

    Ok(HttpResponse::Ok().json(collection))
}

#[post("/api/collections")]
pub async fn insert_collection(pool: web::Data<DbPool>, form: web::Form<NewCollection>) -> Result<HttpResponse, Error> {
    let collection = form.into_inner();

    let response = web::block(move || {
        let mut conn = pool.get()?;

        collections::insert_collection(&mut conn, collection)
    })
    .await?
    .map_err(actix_web::error::ErrorInternalServerError)?;

    Ok(HttpResponse::Ok().json(response))
}

#[delete("/api/collections/{id}")]
pub async fn delete_collection(pool: web::Data<DbPool>, id: web::Path<String>) -> Result<HttpResponse, Error> {
    let id_str = id.into_inner();

    let response = web::block(move || {
        let mut conn = pool.get()?;

        collections::delete_collection(&mut conn, &id_str)
    })
    .await?
    .map_err(actix_web::error::ErrorInternalServerError)?;

    Ok(HttpResponse::Ok().json(response))
}