// @generated automatically by Diesel CLI.

diesel::table! {
    posts (id) {
        id -> Integer,
        #[max_length = 255]
        title -> Varchar,
        body -> Text,
        published -> Bool,
    }
}

diesel::table! {
    privileges (userId, roleId) {
        userId -> Integer,
        roleId -> Integer,
    }
}

diesel::table! {
    role (id) {
        id -> Integer,
        #[max_length = 60]
        name -> Varchar,
    }
}

diesel::table! {
    user (id) {
        id -> Integer,
        #[max_length = 255]
        email -> Varchar,
        #[max_length = 255]
        displayName -> Varchar,
        #[max_length = 255]
        password -> Varchar,
        #[max_length = 255]
        username -> Varchar,
    }
}

diesel::joinable!(privileges -> role (roleId));
diesel::joinable!(privileges -> user (userId));

diesel::allow_tables_to_appear_in_same_query!(
    posts,
    privileges,
    role,
    user,
);
