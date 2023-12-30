use diesel::prelude::*;
use crate::api::resources::products::models::Product;
use crate::db;
use crate::schema::wishes::table as wishes_table;
use crate::schema::products::table as products_table;
use super::models::{Wish, NewWish, WishProduct};
use diesel::result::Error;
use crate::schema::wishes as wishes_schema;

pub fn list_wishes_from_wishlist(wishlist_id: i32) -> Result<Vec<Wish>, Error> {
    let conn = &mut db::establish_connection();

    wishes_table
        .filter(wishes_schema::wishlist_id.eq(wishlist_id))
        .select(Wish::as_select())
        .load(conn)
}

pub fn insert_wish(new_wish: NewWish) -> Result<Wish, Error> {
    let conn = &mut db::establish_connection();

    diesel::insert_into(wishes_table)
        .values(&new_wish)
        .returning(Wish::as_returning())
        .get_result(conn)
}

pub fn find_wish(id: i32) -> Result<WishProduct, Error> {
    let conn = &mut db::establish_connection();

    let wish = wishes_table
        .find(id)
        .select(Wish::as_select())
        .get_result(conn)?;

    let product = products_table
        .find(wish.product_id)
        .select(Product::as_select())
        .get_result(conn)?;

    Ok(WishProduct::from(wish, product))
}