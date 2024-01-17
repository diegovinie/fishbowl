use std::sync::Arc;
use salvo::prelude::*;
use salvo::http::form::FormData;
use crate::api::errors::ApiResult;
use crate::api::utils::pagination::Pagination;
use crate::api::{utils, responses as api_responses, errors as api_errors};
use crate::database::contracts::{DatabaseService, ProductRepo};
use crate::models::Updatable;
use super::models::NewProduct;
use super::repo;

#[handler]
pub fn list_products(req: &mut Request, _depot: &mut Depot, res: &mut Response) {
    match req.query::<i64>("per_page") {
        None => match repo::list_products() {
            Err(error) => api_errors::render_db_retrieving_error(res, error, "products"),

            Ok(products) => api_responses::render_collection(res, products)
        },

        Some(per_page) => {
            let page = req.query::<i64>("page").unwrap_or(1);

            match repo::list_products_paginate(page, per_page) {
                Err(error) => api_errors::render_db_retrieving_error(res, error, "products"),

                Ok((entries, products)) =>
                    api_responses::render_collection_paginated(res, products, Pagination::new(page, per_page, entries)),
            }
        }
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
pub fn show_product(req: &Request, depot: &Depot, res: &mut Response) -> ApiResult<()> {
    let repo = get_repo(depot)?;

    let id = utils::get_req_param::<i32>(req, "id")?;

    let product = repo.find_product(id)?;

    api_responses::render_resource(res, product);

    Ok(())
}

#[handler]
pub fn remove_product(req: &mut Request, _depot: &mut Depot, res: &mut Response) {
    match utils::get_req_param(req, "id") {
        Err(error) => api_errors::render_parse_field_error(res, error, "id"),

        Ok(id) => match repo::delete_product(id) {
            Err(error) => api_errors::render_db_delete_error(res, error, "product"),

            Ok(total_deleted) => match total_deleted {
                0 => api_errors::render_resource_not_found(res, "product"),

                _other => api_responses::render_db_execution(res, total_deleted)
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
        .map(|n| n.to_string())
        .ok_or(FieldNotFound("name"))?;

    let description = form_data.fields.get("description")
        .map(|d| d.to_string());

    let url = form_data.fields.get("url")
        .map(|u| u.to_string());

    let price: f32 = form_data.fields.get("price")
        .ok_or(FieldNotFound("price"))?
        .parse()
        .map_err(|_| ParseFloatErr("price"))?;

    let new_product = NewProduct { name, description, url, price, available: false };

    Ok(new_product)
}

fn get_repo(depot: &Depot) -> ApiResult<Box<dyn ProductRepo>> {
    use crate::api::errors::{ApiError, InjectionError};

    let service = depot.obtain::<Arc<dyn DatabaseService>>()
        .map_err(|_| ApiError::Injection(InjectionError))?;

    Ok(service.clone().product_repo())
}