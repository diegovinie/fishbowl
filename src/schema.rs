// @generated automatically by Diesel CLI.

diesel::table! {
    products (id) {
        id -> Int4,
        name -> Varchar,
        description -> Text,
        url -> Nullable<Varchar>,
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

diesel::table! {
    wishlists (id) {
        id -> Int4,
        title -> Varchar,
        description -> Nullable<Varchar>,
        date -> Nullable<Timestamp>,
        user_id -> Int4,
        published -> Bool,
    }
}

diesel::joinable!(wishlists -> users (user_id));

diesel::allow_tables_to_appear_in_same_query!(
    products,
    users,
    wishlists,
);
