pub mod auth;
pub mod errors;
pub mod resources;
pub mod responses;

pub mod utils {
    use salvo::prelude::*;
    use std::str::FromStr;
    use super::auth::JwtClaims;

    pub fn get_req_param<T: FromStr>(req: &Request, param: &str) -> Result<T, T::Err> {
        req.params()
            .get(param)
            .cloned()
            .unwrap_or_default()
            .parse()
    }

    pub fn get_user_id(depot: &mut Depot) -> Option<i32> {
        match depot.jwt_auth_data::<JwtClaims>() {
            None => None,
            Some(data) => Some(data.claims.id),
        }
    }
}


pub mod users {
    pub mod models {use diesel::prelude::*;

        #[derive(Queryable, Identifiable, Selectable, Debug, PartialEq)]
        #[diesel(table_name = crate::schema::users)]
        #[diesel(check_for_backend(diesel::pg::Pg))]
        pub struct User {
            pub id: i32,
            pub name: String,
            pub email: String,
            pub active: bool,
        }
    }

    pub mod repo {
        use diesel::prelude::*;
        use diesel::{result::Error, SelectableHelper};
        use crate::schema::users::table as users_table;
        use crate::db;
        use super::models::User;


        pub fn find_user(id: i32) -> Result<User, Error> {
            let conn = &mut db::establish_connection();

            users_table
                .find(id)
                .select(User::as_select())
                .first(conn)
        }
    }
}