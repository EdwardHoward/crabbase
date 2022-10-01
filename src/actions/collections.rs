use diesel::prelude::*;

use crate::models::Collection;
use crate::schema::_collections::dsl::*;

type DbError = Box<dyn std::error::Error + Send + Sync>;

pub fn get_collections(conn: &mut SqliteConnection) -> Result<Vec<Collection>, DbError>{
  let collections = _collections
    .limit(10)
    .load::<Collection>(conn)
    .expect("Error loading collections");

  Ok(collections)
}

pub fn get_collection_by_id(conn: &mut SqliteConnection, collection_id: &str) -> Result<Collection, DbError> {
  let collection: Collection = _collections
    .filter(id.eq(collection_id))
    .first(conn)
    .expect("Error loading collection");

  Ok(collection)
}