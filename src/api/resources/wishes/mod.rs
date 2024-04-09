mod controllers;
pub mod models;
pub mod repo;

use salvo::prelude::*;
use self::controllers::{create_wish, list_wishes, show_wish, delete_wish};
use crate::api::auth;
use auth::controllers::handle_auth;
use super::sponsors::controllers::list_sponsors_wish;

pub fn get_root_router() -> Router {
    Router::with_path("wishes")
        .hoop(handle_auth)
        .push(Router::with_path("<wish_id>")
            .push(Router::with_path("sponsors")
                .get(list_sponsors_wish)
            )
        )
}

pub fn get_router_for_wishlists() -> Router {
    Router::with_path("wishlists/<wishlist_id>/wishes")
        .hoop(handle_auth)
        .get(list_wishes)
        .post(create_wish)
        .push(Router::with_path("<id>")
            .get(show_wish)
            .delete(delete_wish)
        )
}
