use diesel::{allow_tables_to_appear_in_same_query, joinable, table};

table! {
    users (id) {
        id -> Int4,
        username -> Varchar,
        password -> Varchar,
        display_name -> Varchar,
        email -> Varchar,
    }
}
