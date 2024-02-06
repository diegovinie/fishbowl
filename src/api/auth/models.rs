use diesel::prelude::*;
use serde::{Deserialize, Serialize};

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

#[derive(Debug, Serialize, Deserialize)]
pub struct ActivateUserClaims {
    pub id: i32,
    pub email: String,
    pub action: ActivateUserAction,
    pub exp: i64,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum ActivateUserAction {
    Activate,
}
