use actix_web::{delete, get, post, put, web, Error, HttpResponse};
use diesel::prelude::*;
use diesel::r2d2::{self, ConnectionManager};
use crate::models::collection::{Collection, CollectionMessage};

type DbPool = r2d2::Pool<ConnectionManager<SqliteConnection>>;

#[get("/api/collections")]
async fn find_all(pool: web::Data<DbPool>) -> Result<HttpResponse, Error> {
    let collections = web::block(move || {
        let mut conn = pool.get()?;

        Collection::find_all(&mut conn)
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
async fn find(pool: web::Data<DbPool>, id: web::Path<String>) -> Result<HttpResponse, Error> {
    let collection = web::block(move || {
      let mut conn = pool.get()?;

      Collection::find(&mut conn, id.into_inner())
    })
    .await?
    .map_err(actix_web::error::ErrorInternalServerError)?;

    Ok(HttpResponse::Ok().json(collection))
}

#[post("/api/collections")]
async fn insert(pool: web::Data<DbPool>, form: web::Form<CollectionMessage>) -> Result<HttpResponse, Error> {
    let collection = form.into_inner();

    let response = web::block(move || {
        let mut conn = pool.get()?;

        Collection::insert(&mut conn, collection)
    })
    .await?
    .map_err(actix_web::error::ErrorInternalServerError)?;

    Ok(HttpResponse::Ok().json(response))
}

#[put("/api/collections/{id}")]
async fn update(pool: web::Data<DbPool>, id: web::Path<String>, form: web::Json<CollectionMessage>) -> Result<HttpResponse, Error> {
    let collection = form.into_inner();
    let id = id.into_inner();

    let response = web::block(move || {
        let mut conn = pool.get()?;

        Collection::update(&mut conn, id, collection)
    })
    .await?
    .map_err(actix_web::error::ErrorInternalServerError)?;

    Ok(HttpResponse::Ok().json(response))
}

#[delete("/api/collections/{id}")]
async fn delete(pool: web::Data<DbPool>, id: web::Path<String>) -> Result<HttpResponse, Error> {
    let response = web::block(move || {
        let mut conn = pool.get()?;

        Collection::delete(&mut conn, id.into_inner())
    })
    .await?
    .map_err(actix_web::error::ErrorInternalServerError)?;

    Ok(HttpResponse::Ok().json(response))
}

pub fn init_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(find);
    cfg.service(find_all);
    cfg.service(insert);
    cfg.service(update);
    cfg.service(delete);
}