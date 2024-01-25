pub mod controllers;
pub mod repo;
pub mod models;

use salvo::Router;
use crate::api::auth;
use self::controllers::{add_sponsor};

pub fn get_router() -> Router {
    Router::with_path("wishes/<wish_id>/sponsors")
        .hoop(auth::handle_auth)
        .post(add_sponsor)
}