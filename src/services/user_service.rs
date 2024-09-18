use crate::models::User;
use bcrypt::{hash, verify};
use diesel::prelude::*;
use diesel::result::Error;

pub async fn find_by_username(username: &str) -> Result<Option<User>, Error> {
    use crate::schema::users::dsl::*;

    let connection = establish_connection();
    let user = users.filter(username.eq(username))
        .first::<User>(&connection)
        .optional()?;

    Ok(user)
}

pub async fn verify_password(user: &User, password: &str) -> Result<bool, Error> {
    let valid = verify(password, &user.password)?;
    Ok(valid)
}

pub async fn create_user(
    username: &str,
    password: &str,
    display_name: &str,
    email: &str,
) -> Result<User, Error> {
    use crate::schema::users;

    let hashed_password = hash(password, 10)?;
    let new_user = NewUser {
        username: username.to_string(),
        password: hashed_password,
        display_name: display_name.to_string(),
        email: email.to_string(),
    };

    let connection = establish_connection();
    diesel::insert_into(users::table)
        .values(&new_user)
        .get_result(&connection)
}
