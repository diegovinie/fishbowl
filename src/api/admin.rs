pub mod controllers;

use salvo::prelude::*;
use self::controllers::populate_products;
use crate::api::auth;

pub fn get_router() -> Router {
    Router::with_path("admin")
        .hoop(auth::handle_auth)
        .path("populate/products")
        .post(populate_products)
}
