use crate::schema::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Queryable)]
pub struct Collection {
    pub id: String,
    pub name: String,
}

#[derive(Debug, Insertable)]
#[diesel(table_name = _collections)]
pub struct CollectionNew<'a> {
    pub name: &'a str,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CollectionJson {
    pub name: String,
}
