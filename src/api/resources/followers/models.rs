use diesel::prelude::*;
use serde::Serialize;
use crate::api::resources::users::models::User;
use crate::api::resources::wishlists::models::Wishlist;
use crate::schema;


#[derive(Serialize, Debug, Clone)]
#[derive(Queryable, Selectable, Identifiable, Associations, PartialEq, AsChangeset)]
#[diesel(belongs_to(Wishlist))]
#[diesel(belongs_to(User))]
#[diesel(table_name = crate::schema::followers)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Follower {
    pub id: i32,
    pub wishlist_id: i32,
    pub user_id: i32,
    pub active: bool,
}

#[derive(Debug, PartialEq, PartialOrd)]
#[derive(Insertable)]
#[diesel(table_name = schema::followers)]
pub struct NewFollower {
    pub wishlist_id: i32,
    pub user_id: i32,
    pub active: bool,
}
