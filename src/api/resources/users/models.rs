use diesel::prelude::*;
use serde::Serialize;
use crate::schema;

#[derive(Serialize, Clone)]
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

#[derive(Debug)]
#[derive(Insertable)]
#[diesel(table_name = schema::users)]
pub struct NewUser {
    pub name: String,
    pub email: String,
    pub password: String,
    pub active: bool,
}