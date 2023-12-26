use diesel::{prelude::*, QueryDsl};
use crate::db;
use crate::schema::users::{table as users_table, dsl::*};
use super::models::User;

pub fn validate(email_candidate: &str, password_candidate: &str) -> bool {
    let conn = &mut db::establish_connection();

    let user_result = QueryDsl::filter(users_table, email.eq(email_candidate))
        .select(User::as_select())
        .first(conn);

    match user_result {
        Err(_) => false,
        Ok(user) => &user.password == password_candidate,
    }
}
