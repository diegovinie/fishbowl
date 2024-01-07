use diesel::prelude::*;
use serde::Serialize;
use salvo::http::form::FormData;
use crate::schema::wishlists;
use crate::api::resources::wishes::models::WishProduct;
use std::time::SystemTime;
use crate::api::users::models::User;
use crate::models::Updatable;

#[derive(
    Serialize,
    Debug,
    Clone,
    Queryable,
    Selectable,
    Identifiable,
    Associations,
    PartialEq,
    AsChangeset,
)]
#[diesel(belongs_to(User))]
#[diesel(table_name = crate::schema::wishlists)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Wishlist {
    pub id: i32,
    pub title: String,
    pub description: Option<String>,
    pub date: Option<SystemTime>,
    pub user_id: i32,
    pub published: bool,
}

#[derive(Debug, Insertable)]
#[diesel(table_name = wishlists)]
pub struct NewWishlist<'a> {
    pub title: &'a str,
    pub description: Option<&'a str>,
    // pub date: Option<SystemTime>,
    pub user_id: i32,
    // pub published: bool,
}

impl Updatable for Wishlist {
    fn merge(&self, form_data: &FormData) -> Self {
        Self {
            id: self.id,
            title: form_data
                .fields
                .get("title")
                .unwrap_or(&self.title)
                .to_string(),
            description: form_data.fields.get("description").map(|d| d.to_string()),
            date: self.date,
            user_id: self.user_id,
            published: self.published,
        }
    }
}

#[derive(Serialize, Queryable, Selectable)]
#[diesel(table_name = crate::schema::wishlists)]
pub struct ListedWishlist {
    pub id: i32,
    pub title: String,
    pub date: Option<SystemTime>,
}

#[derive(Serialize)]
pub struct DetailedWishlist {
    pub id: i32,
    pub title: String,
    pub description: Option<String>,
    pub date: Option<SystemTime>,
    pub user_id: i32,
    pub published: bool,
    pub wishes: Vec<WishProduct>,
}

impl DetailedWishlist {
    pub fn from(wishlist: Wishlist, wishes: Vec<WishProduct>) -> Self {
        Self {
            id: wishlist.id,
            title: wishlist.title,
            description: wishlist.description,
            date: wishlist.date,
            user_id: wishlist.user_id,
            published: wishlist.published,
            wishes,
        }
    }
}
