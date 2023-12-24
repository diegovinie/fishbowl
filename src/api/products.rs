mod models;
mod controllers;
mod repo;

use salvo::prelude::*;
use self::controllers::{list_products, add_product, show_product, remove_product, update_product};


pub fn get_router() -> Router {
    Router::with_path("products")
        .get(list_products)
        .post(add_product)
        .push(Router::with_path("<id>")
            .get(show_product)
            .delete(remove_product)
            .put(update_product)
        )     
}