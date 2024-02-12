use diesel::prelude::*;
use crate::api::utils::pagination::Paginate;
use crate::services::database::{contracts::WishlistRepo, establish_connection};
use crate::db;
use crate::schema::wishlists::table as wishlists_table;
use super::models::DetailedWishlist;
use super::models::{ListedWishlist, NewWishlist, Wishlist};
use diesel::result::Error;
use crate::schema::wishlists as wishlist_schema;
use crate::models::Composable;

pub struct Repo;

impl WishlistRepo for Repo {
    fn find_one(&self, id: i32) -> Result<Wishlist, Error> {
        let conn = &mut establish_connection();

        wishlists_table
        .find(id)
        .select(Wishlist::as_select())
        .first(conn)
    }

    fn insert(&self, new_wishlist: NewWishlist) -> Result<Wishlist, Error> {
        let conn = &mut establish_connection();

        diesel::insert_into(wishlists_table)
            .values(&new_wishlist)
            .returning(Wishlist::as_returning())
            .get_result(conn)        
    }

    fn insert_many(&self, wishlists: Vec<NewWishlist>) -> Result<usize, Error> {
        let conn = &mut establish_connection();

        diesel::insert_into(wishlists_table)
            .values(wishlists)
            .execute(conn)
    }
}

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

pub fn list_wishlists() -> Result<Vec<ListedWishlist>, Error> {
    let conn = &mut db::establish_connection();

    wishlists_table
        .filter(wishlist_schema::published.eq(true))
        .select(ListedWishlist::as_select())
        .load(conn)
}

pub fn list_wishlists_paginate(page: i64, per_page: i64) -> Result<(i64, Vec<ListedWishlist>), Error> {
    let conn = &mut db::establish_connection();

    let results: Vec<(Wishlist, i64)> = wishlists_table
        .filter(wishlist_schema::published.eq(true))
        .paginate(page)
        .per_page(per_page)
        .get_results(conn)?;

        match results.first() {
            None => Ok((0, vec![])),
            Some((_, entries)) => Ok((
                *entries,
                results.into_iter()
                    .map(|(w, _)| ListedWishlist::from(w))
                    .collect())
                )
    }
}

pub fn list_user_wishlists(user_id: i32) -> Result<Vec<ListedWishlist>, Error> {
    let conn = &mut db::establish_connection();

    wishlists_table
        .filter(wishlist_schema::user_id.eq(user_id))
        .select(ListedWishlist::as_select())
        .load(conn)
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
