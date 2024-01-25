use diesel::prelude::*;
use serde::Serialize;
use crate::api::resources::users::models::User;
use crate::api::resources::wishes::models::Wish;
use crate::schema;

#[derive(Serialize, Debug, Clone)]
#[derive(Queryable, Selectable, Identifiable, Associations, PartialEq, AsChangeset)]
#[diesel(belongs_to(User))]
#[diesel(belongs_to(Wish))]
#[diesel(table_name = schema::sponsors)]
pub struct Sponsor {
    id: i32,
    leader: bool,
    amount: f32,
    user_id: i32,
    wish_id: i32,
}

#[derive(Serialize, Debug, Clone)]
pub struct DetailedSponsor {
    id: i32,
    leader: bool,
    amount: f32,
    user: User,
    wish: Wish,
}

#[derive(Debug, PartialEq, PartialOrd)]
#[derive(Insertable)]
#[diesel(table_name = schema::sponsors)]
pub struct NewSponsor {
    pub leader: Option<bool>,
    pub amount: f32,
    pub user_id: i32,
    pub wish_id: i32,
}
