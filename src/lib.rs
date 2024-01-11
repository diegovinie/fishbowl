pub mod schema;
pub mod api;
pub mod home;

pub mod db {
    use diesel::pg::PgConnection;
    use diesel::prelude::*;
    use dotenvy::dotenv;
    use std::env;

    pub fn establish_connection() -> PgConnection {
        dotenv().ok();

        let database_url = env::var("DATABASE_URL")
            .expect("DATABASE_URL must be set");

        PgConnection::establish(&database_url)
            .unwrap_or_else(|_| panic!("Error connecting to {}", database_url))
    }
}

pub mod models {
    use salvo::http::form::FormData;
    use serde::{Deserialize, Serialize};

    pub trait Updatable {
        fn merge(&self, form_data: &FormData) -> Self;
    }

    pub trait Composable<T, G> {
        fn compose(tree: T, branch: G) -> Self;
    }


    #[derive(Deserialize, Serialize, Debug)]
    pub enum Role {
        Admin,
        User,
    }

    impl<'a> From<&'a str> for Role {
        fn from(value: &'a str) -> Self {
            match value {
                "ADMIN" => Role::Admin,
                _other => Role::User,
            }
        }
    }
}