use diesel::prelude::*;
use crate::db;
use crate::schema::wishlists::table as wishlists_table;
use super::models::DetailedWishlist;
use super::models::{ListedWishlist, NewWishlist, Wishlist};
use diesel::result::Error;
use crate::schema::wishlists as wishlist_schema;
use crate::models::Composable;

pub fn find_wishlist(id: i32, user_id: i32) -> Result<Wishlist, Error> {
    let conn = &mut db::establish_connection();

    wishlists_table
        .filter(wishlist_schema::user_id.eq(user_id))
        .find(id)
        .select(Wishlist::as_select())
        .first(conn)
}

pub fn find_detailed_wishlist(id: i32, user_id: i32) -> Result<DetailedWishlist, Error> {
    use crate::api::resources::wishes::repo as wishes_repo;

    let wishlist = find_wishlist(id, user_id)?;

    let wishes = wishes_repo::list_detailed_wishes(id)?;

    Ok(DetailedWishlist::compose(wishlist, wishes))
}

pub fn list_wishlists(user_id: i32) -> Result<Vec<ListedWishlist>, Error> {
    let conn = &mut db::establish_connection();

    wishlists_table
        .filter(wishlist_schema::user_id.eq(user_id))
        .select(ListedWishlist::as_select())
        .load(conn)
}

pub fn insert_wishist(new_wishlist: NewWishlist) -> Result<Wishlist, Error> {
    let conn = &mut db::establish_connection();

    diesel::insert_into(wishlists_table)
        .values(&new_wishlist)
        .returning(Wishlist::as_returning())
        .get_result(conn)
}

pub fn update_wishist(wishlist: &Wishlist, user_id: i32) -> Result<Wishlist, Error> {
    let conn = &mut db::establish_connection();

    diesel::update(
        wishlists_table
            .filter(wishlist_schema::user_id.eq(user_id))
            .find(wishlist.id),
    )
    .set(wishlist)
    .get_result(conn)
}

pub fn delete_wishlist(id: i32, user_id: i32) -> Result<usize, Error> {
    let conn = &mut db::establish_connection();

    diesel::delete(
        wishlists_table
            .filter(wishlist_schema::user_id.eq(user_id))
            .find(id),
    )
    .execute(conn)
}
