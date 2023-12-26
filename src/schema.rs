// @generated automatically by Diesel CLI.

diesel::table! {
    products (id) {
        id -> Int4,
        name -> Varchar,
        description -> Text,
        price -> Float4,
        available -> Bool,
    }
}

diesel::table! {
    users (id) {
        id -> Int4,
        name -> Varchar,
        email -> Text,
        password -> Text,
        active -> Bool,
    }
}

diesel::allow_tables_to_appear_in_same_query!(
    products,
    users,
);
