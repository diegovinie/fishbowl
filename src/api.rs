pub mod auth;
pub mod errors;
pub mod resources;
pub mod responses;
pub mod admin;
pub mod utils;
pub mod validations;

pub fn get_router() -> salvo::Router {
    use resources::{products, wishlists, wishes, sponsors};

    salvo::Router::with_path("api")
        .path("v1")
        .push(admin::get_router())
        .push(products::get_router())
        .push(wishlists::get_router())
        .push(wishes::get_router_for_wishlists())
        .push(wishes::get_root_router())
        .push(sponsors::get_router())
}