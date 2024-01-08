use diesel::prelude::*;
use serde::Serialize;
use crate::api::resources::wishlists::models::Wishlist;
use crate::api::resources::products::models::Product;
use crate::models::Composable;

#[derive(Serialize, Debug, Clone)]
#[derive(Queryable, Selectable, Identifiable, Associations, PartialEq, AsChangeset)]
#[diesel(belongs_to(Wishlist))]
#[diesel(belongs_to(Product))]
#[diesel(table_name = crate::schema::wishes)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Wish {
    pub id: i32,
    pub wishlist_id: i32,
    pub product_id: i32,
    pub pending: bool,
}

#[derive(Debug)]
#[derive(Insertable)]
#[diesel(table_name = crate::schema::wishes)]
pub struct NewWish {
    pub wishlist_id: i32,
    pub product_id: i32,
}

#[derive(Serialize)]
pub struct WishProduct {
    pub id: i32,
    pub wishlist_id: i32,
    pub product: Product,
    pub pending: bool,
}

impl Composable<Wish, Product> for WishProduct {
    fn compose(wish: Wish, product: Product) -> Self {
        let Wish { id, wishlist_id, pending, .. } = wish;

        Self { id, wishlist_id, pending, product }
    }
}