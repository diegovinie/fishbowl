use salvo::prelude::*;
    
#[handler]
pub fn list_products() -> &'static str {
    "list products"
}

#[handler]
pub fn add_product() {

}

#[handler]
pub fn show_product() {

}

#[handler]
pub fn remove_product() {

}

#[handler]
pub fn update_product() {

}

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