use salvo::prelude::*;
use salvo::http::form::FormData;
use crate::api::{utils, responses as api_responses, errors as api_errors};
use crate::models::Updatable;
use super::models::NewProduct;
use super::repo;

#[handler]
pub fn list_products(_req: &mut Request, _depot: &mut Depot, res: &mut Response) {
    match repo::list_products() {
        Err(error) => api_errors::render_db_retrieving_error(res, error, "products"),

        Ok(products) => api_responses::render_collection(res, products)
    }
}

#[handler]
pub async fn add_product(req: &mut Request, _depot: &mut Depot, res: &mut Response) {
    match req.form_data().await {
        Err(error) => api_errors::render_form_data_error(res, error),

        Ok(form_data) => match cast_form_data_to_new_product(form_data) {
            Err(error) => api_errors::render_cast_error(res, error),

            Ok(new_product) => match repo::insert_product(new_product) {
                Err(error) => api_errors::render_db_insert_error(res, error, "product"),

                Ok(product) => api_responses::render_resource_created(res, product)
            }
        }
    }
}

#[handler]
pub fn show_product(req: &mut Request, _depot: &mut Depot, res: &mut Response) {
    match utils::get_req_param::<i32>(req, "id") {
        Err(error) => api_errors::render_parse_field_error(res, error, "id"),

        Ok(id) => match repo::find_product(id) {
            Err(_) => api_errors::render_resource_not_found(res, "product"),

            Ok(product) => api_responses::render_resource(res, product)
        }
    }
}

#[handler]
pub fn remove_product(req: &mut Request, _depot: &mut Depot, res: &mut Response) {
    match utils::get_req_param(req, "id") {
        Err(error) => api_errors::render_parse_field_error(res, error, "id"),

        Ok(id) => match repo::delete_product(id) {
            Err(error) => api_errors::render_db_delete_error(res, error, "product"),

            Ok(total_deleted) => match total_deleted {
                0 => api_errors::render_resource_not_found(res, "product"),

                _other => api_responses::render_resource_deleted(res, total_deleted)
            }
        }
    }
}

#[handler]
pub async fn update_product(req: &mut Request, _depot: &mut Depot, res: &mut Response) {
    match utils::get_req_param(req, "id") {
        Err(error) => api_errors::render_parse_field_error(res, error, "id"),

        Ok(id) => match req.form_data().await {
            Err(error) => api_errors::render_form_data_error(res, error),

            Ok(form_data) => match repo::find_product(id) {
                Err(_) => api_errors::render_resource_not_found(res, "products"),

                Ok(product) => {
                    let product_updated = product.merge(form_data);

                    match repo::update_product(&product_updated) {
                        Err(error) => api_errors::render_db_update_error(res, error, "product"),

                        Ok(updated_product) => api_responses::render_resource_updated(res, updated_product)
                    }
                }
            }
        }
    }
}

fn cast_form_data_to_new_product(form_data: &FormData) -> Result<NewProduct, api_errors::Error> {
    use api_errors::Error::{FieldNotFound, ParseFloatErr};

    let name = form_data.fields.get("name")
        .ok_or(FieldNotFound("name"))?;

    let description = form_data.fields.get("description")
        .ok_or(FieldNotFound("description"))?;

    let price: f32 = form_data.fields.get("price")
        .ok_or(FieldNotFound("price"))?
        .parse()
        .map_err(|_| ParseFloatErr("price"))?;

    let new_product = NewProduct { name, description, price, available: false };

    Ok(new_product)
}
