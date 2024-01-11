pub mod models;
pub mod controllers;
pub mod repo;

use salvo::prelude::*;
use crate::api::auth;
use self::controllers::{
    list_user_wishlists, 
    list_wishlists,
    create_wishlist,
    delete_wishlist,
    show_wishlist, 
    update_wishlist, 
};

pub fn get_router() -> Router {
    Router::with_path("wishlists")
        .hoop(auth::handle_auth)
        .get(list_wishlists)
        .post(create_wishlist)
        .push(Router::with_path("user")
            .get(list_user_wishlists)
        )
        .push(Router::with_path("<id>")
            .get(show_wishlist)
            .delete(delete_wishlist)
            .put(update_wishlist)
        )
}