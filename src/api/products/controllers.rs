use salvo::prelude::*;
use salvo::http::form::FormData;
use crate::api::utils;
use super::models::NewProduct;
use super::repo;

#[derive(Debug)]
enum Error<'a> {
    FieldNotFound(&'a str),
    ParseFloatErr(&'a str),
}
    
#[handler]
pub fn list_products(_req: &mut Request, _depot: &mut Depot, res: &mut Response) {
    let products = repo::list_products()
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

    let product = repo::insert_product(new_product)
        .expect("Error inserting product");

    res.render(Json(product))
}

#[handler]
pub fn show_product(req: &mut Request, _depot: &mut Depot, res: &mut Response) {
    let id: i32 = utils::get_req_param(req, "id")
        .unwrap_or_default();

    let product = repo::find_product(id)
        .expect("Product not found");

    res.render(Json(product))
}

#[handler]
pub fn remove_product(req: &mut Request, _depot: &mut Depot, res: &mut Response) {
    let id: i32 = utils::get_req_param(req, "id")
        .unwrap_or_default();
    
    let total_deleted = repo::delete_product(id)
        .expect("Failed deleting");

    res.render(format!("Total deleted: {}", total_deleted))
}

#[handler]
pub fn update_product(req: &mut Request, _depot: &mut Depot, res: &mut Response) {
    let id: i32 = utils::get_req_param(req, "id")
        .unwrap_or_default();

    let product = repo::find_product(id)
        .expect("Product not found");

    res.render(Json(product))
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
