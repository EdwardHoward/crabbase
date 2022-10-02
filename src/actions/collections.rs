use diesel::prelude::*;

use crate::models::collection::{ Collection, CollectionMessage };
use crate::schema::_collections::dsl::*;

type DbError = Box<dyn std::error::Error + Send + Sync>;

pub fn get_collections(conn: &mut SqliteConnection) -> Result<Vec<Collection>, DbError>{
  let collections = _collections
    .limit(10)
    .load::<Collection>(conn);

  return match collections {
    Ok(collections) => Ok(collections),
    Err(error) => Err(Box::new(error))
  }
}

pub fn get_collection(conn: &mut SqliteConnection, collection_id: &str) -> Result<Collection, DbError> {
  let collection = _collections
    .filter(id.eq(collection_id))
    .first::<Collection>(conn);

  return match collection {
    Ok(collection) => Ok(collection),
    Err(error) => Err(Box::new(error))
  }
}

pub fn insert_collection(conn: &mut SqliteConnection, collection: CollectionMessage) -> Result<usize, DbError> {
  let new_collection = Collection::from(collection);

  let rows_inserted = diesel::insert_into(_collections)
    .values(&new_collection)
    .execute(conn);

  return match rows_inserted {
    Ok(response) => Ok(response),
    Err(error) => Err(Box::new(error))
  }
}

pub fn delete_collection(conn: &mut SqliteConnection, collection_id: &str) -> Result<usize, DbError> {
  let rows_deleted = diesel::delete(
    _collections.filter(
      id.eq(collection_id)
    )
  ).execute(conn);

  return match rows_deleted {
    Ok(collection) => Ok(collection),
    Err(error) => Err(Box::new(error))
  }
}