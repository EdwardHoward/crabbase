use diesel::prelude::*;
use serde::{Deserialize, Serialize};

use crate::schema::*;
use crate::utils::generate_id;

type DbError = Box<dyn std::error::Error + Send + Sync>;

#[derive(Debug, Serialize, Deserialize, Queryable, Insertable)]
#[diesel(table_name = _collections)]
pub struct Collection {
  pub id: String,
  pub name: String,
  pub schema: String,
}

#[derive(Serialize, Deserialize, AsChangeset)]
#[diesel(table_name = _collections)]
pub struct CollectionMessage {
  pub name: String,
  pub schema: String,
}

impl Collection {
  pub fn find_all(conn: &mut SqliteConnection) -> Result<Vec<Self>, DbError> {
    let collections = _collections::table
    .limit(10)
    .load::<Collection>(conn);

    return match collections {
      Ok(collections) => Ok(collections),
      Err(error) => Err(Box::new(error))
    }
  }

  pub fn find(conn: &mut SqliteConnection, id: String) -> Result<Self, DbError> {
    let collection = _collections::table
      .filter(_collections::id.eq(id))
      .first::<Collection>(conn);
  
    return match collection {
      Ok(collection) => Ok(collection),
      Err(error) => Err(Box::new(error))
    }
  }

  pub fn insert(conn: &mut SqliteConnection, collection: CollectionMessage) -> Result<Collection, DbError> {
    let new_collection = Collection::from(collection);

    let rows_inserted = diesel::insert_into(_collections::table)
      .values(&new_collection)
      .execute(conn);
  
    return match rows_inserted {
      Ok(_response) => Ok(new_collection),
      Err(error) => Err(Box::new(error))
    }
  }

  pub fn update(conn: &mut SqliteConnection, id: String, collection: CollectionMessage) -> Result<usize, DbError> {
    let response = diesel::update(_collections::table)
    .filter(_collections::id.eq(id))
    .set(collection)
    .execute(conn);

    return match response {
      Ok(response) => Ok(response),
      Err(error) => Err(Box::new(error))
    }
  }

  pub fn delete(conn: &mut SqliteConnection, id: String)  -> Result<usize, DbError> {
    let rows_deleted = diesel::delete(
      _collections::table.filter(
        _collections::id.eq(id)
      )
    ).execute(conn);
  
    return match rows_deleted {
      Ok(collection) => Ok(collection),
      Err(error) => Err(Box::new(error))
    }
  } 
}

impl From<CollectionMessage> for Collection {
  fn from(collection: CollectionMessage) -> Self {
    Collection {
      id: generate_id(15),
      name: collection.name,
      schema: collection.schema,
    }
  }
}