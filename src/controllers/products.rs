use salvo::prelude::*;
use diesel::prelude::*;
use crate::db;
use crate::models::Product;
use crate::schema::products::table as products_table;
    
#[handler]
pub fn list_products(_req: &mut Request, _depot: &mut Depot, res: &mut Response) {
    let connection = &mut db::establish_connection();
    
    let products = products_table
        .select(Product::as_select())
        .load(connection)
        .expect("Error loading products");

    res.render(Json(products))

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