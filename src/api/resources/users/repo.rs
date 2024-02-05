use diesel::prelude::*;
use diesel::{result::Error, SelectableHelper};
use crate::services::database::{contracts::UserRepo, establish_connection};
use crate::schema::users::table as users_table;
use super::models::{User, NewUser};

pub struct Repo;

impl UserRepo for Repo {
    fn list(&self) -> Result<Vec<User>, Error> {
        let conn = &mut establish_connection();

        users_table
            .select(User::as_select())
            .load(conn)
    }

    fn find_user(&self, id: i32) -> Result<User, Error> {
        let conn = &mut establish_connection();

        users_table
            .find(id)
            .select(User::as_select())
            .first(conn)
    }

    fn insert_many(&self, users: Vec<NewUser>) -> Result<usize, Error> {
        let conn = &mut establish_connection();

        diesel::insert_into(users_table)
            .values(users)
            .execute(conn)
    }

    fn insert(&self, new_user: NewUser) -> Result<User, Error> {
        let conn = &mut establish_connection();
        
        diesel::insert_into(users_table)
            .values(&new_user)
            .returning(User::as_returning())
            .get_result(conn)
    }
}
