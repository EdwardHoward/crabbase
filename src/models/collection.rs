use serde::{Deserialize, Serialize};

use crate::schema::*;
use crate::utils::generate_id;

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

impl From<CollectionMessage> for Collection {
  fn from(collection: CollectionMessage) -> Self {
    Collection {
      id: generate_id(15),
      name: collection.name,
      schema: collection.schema,
    }
  }
}