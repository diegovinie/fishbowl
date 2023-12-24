use fishbowl::models::NewProduct;
use salvo::http::form::FormData;
use salvo::prelude::*;
use diesel::prelude::*;
use crate::db;
use crate::models::Product;
use crate::schema::products::table as products_table;
use super::utils;

#[derive(Debug)]
enum Error<'a> {
    FieldNotFound(&'a str),
    ParseFloatErr(&'a str),
}
    
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
pub async fn add_product(req: &mut Request, _depot: &mut Depot, res: &mut Response) {

    let form_data: &salvo::http::form::FormData = req.form_data()
        .await
        .expect("Error getting the form data");

    let new_product = cast_form_data_to_new_product(form_data)
        .expect("ups");

    res.render(format!("add product: {:?}", new_product));
}

#[handler]
pub fn show_product(req: &mut Request, _depot: &mut Depot, res: &mut Response) {
    let id: i32 = utils::get_req_param(req, "id")
        .unwrap_or_default();

    let connection = &mut db::establish_connection();

    let product = products_table
        .find(id)
        .select(Product::as_select())
        .first(connection)
        .expect("Product not found");

    res.render(Json(product))
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

fn cast_form_data_to_new_product(form_data: &FormData) -> Result<NewProduct, Error> {
    let name = form_data.fields.get("name")
        .ok_or(Error::FieldNotFound("name"))?;

    let description = form_data.fields.get("description")
        .ok_or(Error::FieldNotFound("description"))?;

    let price: f32 = form_data.fields.get("price")
        .ok_or(Error::FieldNotFound("price"))?
        .parse()
        .map_err(|_| Error::ParseFloatErr("price"))?;

    let new_product = NewProduct { name, description, price, available: false };

    Ok(new_product)
}