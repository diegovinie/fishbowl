pub mod auth;
pub mod errors;
pub mod resources;
pub mod responses;
pub mod admin;
pub mod utils;

pub fn get_router() -> salvo::Router {
    use resources::{products, wishlists, wishes};

    salvo::Router::with_path("api")
        .path("v1")
        .push(admin::get_router())
        .push(products::get_router())
        .push(wishlists::get_router())
        .push(wishes::get_router())
}