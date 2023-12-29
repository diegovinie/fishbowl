mod models;
mod controllers;
mod repo;

use salvo::prelude::*;
use crate::api::auth;
use self::controllers::{list_wishlist, create_wishlist, delete_wishlist, show_wishlist, update_wishist};


pub fn get_router() -> Router {
    Router::with_path("wishlists")
        .hoop(auth::handle_auth)
        .get(list_wishlist)
        .post(create_wishlist)
        .push(Router::with_path("<id>")
            .get(show_wishlist)
            .delete(delete_wishlist)
            .put(update_wishist)
        )     
}