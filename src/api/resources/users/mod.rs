pub mod models;

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
