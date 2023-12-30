
mod controllers;
mod models;
mod repo;

use salvo::prelude::*;
use crate::api::auth;
use self::controllers::{list_wishes, create_wish};

pub fn get_router() -> Router {
    Router::with_path("wishlists/<wishlist_id>/wishes")
        .hoop(auth::handle_auth)
        .get(list_wishes)
        .post(create_wish)
}