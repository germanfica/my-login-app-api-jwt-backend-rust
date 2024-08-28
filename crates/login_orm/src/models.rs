use diesel::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Queryable, Selectable, Serialize, Deserialize)]
#[diesel(table_name = crate::schema::posts)]
// #[diesel(check_for_backend(diesel::pg::Pg))]
#[diesel(check_for_backend(diesel::mysql::Mysql))]
pub struct Post {
    pub id: i32,
    // pub id: u64,
    pub title: String,
    pub body: String,
    pub published: bool,
}