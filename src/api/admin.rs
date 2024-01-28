pub mod controllers;

use salvo::prelude::*;
use self::controllers::{check_admin_role, list_users, populate_products, populate_users, populate_wishlists};
use super::auth::controllers::handle_auth;

pub fn get_router() -> Router {
    Router::with_path("admin")
        .hoop(handle_auth)
        .hoop(check_admin_role)
        .push(Router::with_path("users").get(list_users))
        .push(Router::with_path("populate")
            .push(Router::with_path("users").post(populate_users))
            .push(Router::with_path("products").post(populate_products))
            .push(Router::with_path("wishlists").post(populate_wishlists))
        )
}
