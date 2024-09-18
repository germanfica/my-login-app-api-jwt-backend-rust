use diesel::deserialize::FromSql;
use diesel::prelude::*;
use diesel::serialize::ToSql;
use serde::{Deserialize, Serialize};
use crate::schema::*; // Asegúrate de que los módulos correctos estén disponibles

use diesel::prelude::*;
use diesel::sql_types::Text;
use std::io::Write;

// Modelo para la tabla `posts`
#[derive(Queryable, Selectable, Serialize, Deserialize)]
#[diesel(table_name = crate::schema::posts)]
#[diesel(check_for_backend(diesel::mysql::Mysql))]
pub struct Post {
    pub id: i32,
    pub title: String,
    pub body: String,
    pub published: bool,
}

// Modelo para la tabla `user`
#[derive(Queryable, Selectable, Serialize, Deserialize)]
#[diesel(table_name = crate::schema::user)]
#[diesel(check_for_backend(diesel::mysql::Mysql))]
pub struct User {
    pub id: i32,
    pub email: String,
    #[diesel(column_name = displayName)]  // Mapea "displayName" de SQL a "display_name" en Rust
    pub display_name: String,  // Cambiado a "displayName"
    pub password: String,
    pub username: String,
}

// Modelo para la tabla `role`
#[derive(Queryable, Selectable, Serialize, Deserialize)]
#[diesel(table_name = crate::schema::role)]
#[diesel(check_for_backend(diesel::mysql::Mysql))]
pub struct Role {
    pub id: i32,
    //pub name: RoleNameEnum,  // Usar el tipo correcto definido en el esquema
    pub name: String,  // Cambiado a String ya que el tipo personalizado "RoleNameEnum" se maneja como Varchar
}

// Modelo para la tabla `privileges`
#[derive(Queryable, Selectable, Serialize, Deserialize, Associations)]
#[diesel(belongs_to(User, foreign_key = userId))]  // Cambiado a "userId"
#[diesel(belongs_to(Role, foreign_key = roleId))]  // Cambiado a "roleId"
#[diesel(table_name = crate::schema::privileges)]
#[diesel(check_for_backend(diesel::mysql::Mysql))]
pub struct Privilege {
    #[diesel(column_name = userId)]
    pub user_id: i32,  // Cambiado a "userId"
    #[diesel(column_name = roleId)]
    pub role_id: i32,  // Cambiado a "roleId"
}
