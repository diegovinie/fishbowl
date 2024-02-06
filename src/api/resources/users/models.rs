use diesel::prelude::*;
use serde::{Serialize, Deserialize};
use crate::schema;
use crate::api::auth::models::User as AuthUser;

#[derive(Serialize, Deserialize, Clone)]
#[derive(Queryable, Identifiable, Selectable, Debug, PartialEq)]
#[diesel(table_name = crate::schema::users)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct User {
    pub id: i32,
    pub name: String,
    pub role: String,
    pub email: String,
    pub active: bool,
}

impl From<AuthUser> for User {
    fn from(value: AuthUser) -> Self {
        let AuthUser { id, name, email, role, active, .. } = value;

        Self { id, name, role, email, active }
    }
}

#[derive(Debug)]
#[derive(Insertable)]
#[diesel(table_name = schema::users)]
pub struct NewUser {
    pub name: String,
    pub email: String,
    pub password: Vec<u8>,
    pub active: bool,
}
    