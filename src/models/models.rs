use serde::{Serialize, Deserialize};
use diesel::prelude::*;
use diesel::Queryable;

#[derive(Debug, Queryable, Serialize, Deserialize)]
pub struct User {
    pub id: i32,
    pub username: String,
    pub password: String,
    pub display_name: String,
    pub email: String,
}

#[derive(Insertable)]
#[table_name = "users"]
pub struct NewUser {
    pub username: String,
    pub password: String,
    pub display_name: String,
    pub email: String,
}
