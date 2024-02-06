use diesel::{prelude::*, QueryDsl};
use crate::db;
use crate::schema::users::{table as users_table, dsl::*};
use super::models::User;
use crate::api::utils::compare_passwords;

pub fn validate(email_candidate: &str, password_candidate: &str) -> Option<User> {
    let conn = &mut db::establish_connection();

    let user_result = QueryDsl::filter(users_table, email.eq(email_candidate))
        .select(User::as_select())
        .first(conn);

    match user_result {
        Err(_) => None,
        Ok(user) => match compare_passwords(&user.password, password_candidate) {
            false => None,
            true => Some(user),
        }
    }
}

pub fn activate(user_id: i32, user_email: &str) -> Result<usize, diesel::result::Error> {
    let conn = &mut db::establish_connection();

    diesel::update(users_table.filter(id.eq(user_id).and(email.eq(user_email))))
        .set(active.eq(true))
        .execute(conn)
}
