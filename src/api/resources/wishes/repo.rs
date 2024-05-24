use diesel::prelude::*;
use super::models::{NewWish, Wish, WishProduct};
use crate::api::resources::products::models::Product;
use crate::services::database::contracts::WishRepo;
use crate::services::database::establish_connection;
use crate::db;
use crate::schema;
use crate::schema::wishes::table as wishes_table;
use crate::schema::products::table as products_table;
use diesel::result::Error;
use crate::schema::wishes as wishes_schema;
use crate::models::Composable;

pub struct Repo;

impl WishRepo for Repo {
    fn list_by_wishlist(&self, id: i32) -> Result<Vec<WishProduct>, Error> {
        let conn = &mut establish_connection();

        let wish_product_list = wishes_table
            .inner_join(schema::products::table)
            .filter(schema::wishes::wishlist_id.eq(id))
            .select((Wish::as_select(), Product::as_select()))
            .load::<(Wish, Product)>(conn)?;

        let wishes = wish_product_list
            .into_iter()
            .map(|(w, p)| WishProduct::compose(w, p))
            .collect();

        Ok(wishes)
    }
    
    fn insert(&self, new_wish: NewWish) -> Result<Wish, Error> {
        let conn = &mut establish_connection();

        diesel::insert_into(wishes_table)
            .values(&new_wish)
            .returning(Wish::as_returning())
            .get_result(conn)
    }
    
    fn find_one_expanded(&self, id: i32) -> Result<WishProduct, Error> {
        let conn = &mut establish_connection();

        let wish = wishes_table
            .find(id)
            .select(Wish::as_select())
            .get_result(conn)?;

        let product = products_table
            .find(wish.product_id)
            .select(Product::as_select())
            .get_result(conn)?;

        Ok(WishProduct::compose(wish, product))
    }
}

pub fn list_wishes_from_wishlist(wishlist_id: i32) -> Result<Vec<Wish>, Error> {
    let conn = &mut db::establish_connection();

    wishes_table
        .filter(wishes_schema::wishlist_id.eq(wishlist_id))
        .select(Wish::as_select())
        .load(conn)
}

pub fn list_detailed_wishes(id: i32) -> Result<Vec<WishProduct>, Error> {
    let conn = &mut db::establish_connection();

    let wish_product_list = schema::wishes::table
        .inner_join(schema::products::table)
        .filter(schema::wishes::wishlist_id.eq(id))
        .select((Wish::as_select(), Product::as_select()))
        .load::<(Wish, Product)>(conn)?;

    let wishes = wish_product_list
        .into_iter()
        .map(|(w, p)| WishProduct::compose(w, p))
        .collect();

    Ok(wishes)
}

pub fn delete_wish(id: i32) -> Result<usize, Error> {
    let conn = &mut db::establish_connection();

    diesel::delete(schema::wishes::table.find(id))
        .execute(conn)
}
