pub mod controllers;

use salvo::prelude::*;
use self::controllers::{populate_products, populate_users};
use crate::api::auth;

pub fn get_router() -> Router {
    Router::with_path("admin")
        .hoop(auth::handle_auth)
        .path("populate")
        .push(Router::with_path("users").post(populate_users))
        .push(Router::with_path("products").post(populate_products))
}
