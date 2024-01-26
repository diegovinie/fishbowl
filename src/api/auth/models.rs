use diesel::prelude::*;
use serde::Serialize;
use crate::api::resources::users::models::User as ResourceUser;

#[derive(Debug, Clone, Serialize)]
#[derive(Queryable, Selectable, AsChangeset)]
#[diesel(table_name = crate::schema::users)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct User {
    pub id: i32,
    pub name: String,
    pub email: String,
    pub role: String,
    pub password: Vec<u8>,
    pub active: bool,
}

impl Into<ResourceUser> for User {
    fn into(self) -> ResourceUser {
        let Self { id, name, email, role, active, .. } = self;

        ResourceUser { id, name, email, role, active }
    }
}
