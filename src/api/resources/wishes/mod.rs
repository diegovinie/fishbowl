mod controllers;
pub mod models;
pub mod repo;

use salvo::prelude::*;
use self::controllers::{create_wish, list_wishes, show_wish, delete_wish};
use crate::api::auth;
use auth::controllers::handle_auth;

pub fn get_router() -> Router {
    Router::with_path("wishlists/<wishlist_id>/wishes")
        .hoop(handle_auth)
        .get(list_wishes)
        .post(create_wish)
        .push(Router::with_path("<id>")
            .get(show_wish)
            .delete(delete_wish)
        )
}
