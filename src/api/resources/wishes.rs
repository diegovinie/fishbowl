mod controllers;
pub mod models;
pub mod repo;

use salvo::prelude::*;
use self::controllers::{create_wish, list_wishes, show_wish};
use crate::api::auth;

pub fn get_router() -> Router {
    Router::with_path("wishlists/<wishlist_id>/wishes")
        .hoop(auth::handle_auth)
        .get(list_wishes)
        .post(create_wish)
        .push(Router::with_path("<id>")
            .get(show_wish)
        )
}
