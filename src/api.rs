pub mod auth;
pub mod errors;
pub mod resources;
pub mod responses;
pub mod admin;
pub mod utils;

pub mod users {
    pub mod models {
        use diesel::prelude::*;
        use crate::schema;

        #[derive(Queryable, Identifiable, Selectable, Debug, PartialEq)]
        #[diesel(table_name = crate::schema::users)]
        #[diesel(check_for_backend(diesel::pg::Pg))]
        pub struct User {
            pub id: i32,
            pub name: String,
            pub email: String,
            pub active: bool,
        }

        #[derive(Debug)]
        #[derive(Insertable)]
        #[diesel(table_name = schema::users)]
        pub struct NewUser {
            pub name: String,
            pub email: String,
            pub password: String,
            pub active: bool,
        }
    }

    pub mod repo {
        use diesel::prelude::*;
        use diesel::{result::Error, SelectableHelper};
        use crate::schema::users::table as users_table;
        use crate::db;
        use super::models::{User, NewUser};


        pub fn find_user(id: i32) -> Result<User, Error> {
            let conn = &mut db::establish_connection();

            users_table
                .find(id)
                .select(User::as_select())
                .first(conn)
        }

        pub fn insert_batch(users: Vec<NewUser>) -> Result<usize, Error> {
            let conn = &mut db::establish_connection();

            diesel::insert_into(users_table)
                .values(users)
                .execute(conn)
        }
    }
}

pub fn get_router() -> salvo::Router {
    use resources::{products, wishlists, wishes};

    salvo::Router::with_path("api")
        .path("v1")
        .push(admin::get_router())
        .push(products::get_router())
        .push(wishlists::get_router())
        .push(wishes::get_router())
}