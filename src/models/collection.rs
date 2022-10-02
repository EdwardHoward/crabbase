use crate::schema::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Queryable, Insertable)]
#[diesel(table_name = _collections)]
pub struct Collection {
    pub id: String,
    pub name: String,
    pub schema: String,
}

#[derive(Debug, Deserialize)]
pub struct NewCollection {
    pub name: String,
    pub schema: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CollectionJson {
    pub name: String,
    pub schema: String,
}
