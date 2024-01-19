use std::sync::Arc;
use salvo::prelude::*;
use salvo::http::form::FormData;
use crate::api::errors::{ApiResult, ApiError};
use crate::api::utils::pagination::Pagination;
use crate::api::{utils, responses as api_responses, errors as api_errors};
use crate::database::contracts::{DatabaseService, ProductRepo};
use crate::models::Updatable;
use super::models::NewProduct;
use super::repo;

#[handler]
pub fn list_products(req: &mut Request, depot: &Depot, res: &mut Response) -> ApiResult<()> {
    let repo = get_repo(depot)?;

    match req.query::<i64>("per_page") {
        None => {
            let products = repo.list_products()?;

            api_responses::render_collection(res, products);
        },

        Some(per_page) => {
            let page = req.query::<i64>("page").unwrap_or(1);

            let (entries, products) = repo.list_products_paginate(page, per_page)?;

            api_responses::render_collection_paginated(res, products, Pagination::new(page, per_page, entries));
        }
    };

    Ok(())
}

#[handler]
pub async fn add_product(req: &mut Request, depot: &Depot, res: &mut Response) -> ApiResult<()> {
    let repo = get_repo(depot)?;

    let form_data = req.form_data().await?;

    let new_product = cast_form_data_to_new_product(form_data)?;

    let product = repo.insert_product(new_product)?;

    api_responses::render_resource_created(res, product);
    
    Ok(())
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
pub fn remove_product(req: &Request, depot: &Depot, res: &mut Response) {
    let repo = get_repo(depot).unwrap();

    match utils::get_req_param(req, "id") {
        Err(error) => api_errors::render_parse_field_error(res, error, "id"),

        Ok(id) => match repo.delete_product(id) {
            Err(error) => api_errors::render_db_delete_error(res, error, "product"),

            Ok(total_deleted) => match total_deleted {
                0 => api_errors::render_resource_not_found(res, "product"),

                _other => api_responses::render_db_execution(res, total_deleted)
            }
        }
    }
}

#[handler]
pub async fn update_product(req: &mut Request, depot: &Depot, res: &mut Response) {
    let repo = get_repo(depot).unwrap();

    match utils::get_req_param(req, "id") {
        Err(error) => api_errors::render_parse_field_error(res, error, "id"),

        Ok(id) => match req.form_data().await {
            Err(error) => api_errors::render_form_data_error(res, error),

            Ok(form_data) => match repo.find_product(id) {
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

fn cast_form_data_to_new_product(form_data: &FormData) -> Result<NewProduct, ApiError> {
    let name = form_data.fields.get("name")
        .map(|n| n.to_string())
        .ok_or(ApiError::FieldNotFound("name".to_string()))?;

    let description = form_data.fields.get("description")
        .map(|d| d.to_string());

    let url = form_data.fields.get("url")
        .map(|u| u.to_string());

    let price: f32 = form_data.fields.get("price")
        .ok_or(ApiError::FieldNotFound("price".to_string()))?
        .parse()
        .map_err(|error| ApiError::ParseFloat(error, "price".to_string()))?;

    let new_product = NewProduct { name, description, url, price, available: false };

    Ok(new_product)
}

fn get_repo(depot: &Depot) -> ApiResult<Box<dyn ProductRepo>> {
    use crate::api::errors::InjectionError;

    let service = depot.obtain::<Arc<dyn DatabaseService>>()
        .map_err(|_| ApiError::Injection(InjectionError))?;

    Ok(service.clone().product_repo())
}

#[cfg(test)]
mod tests {
    use salvo::http::form::FormData;
    use super::cast_form_data_to_new_product;
    use crate::api::resources::products::models::NewProduct;

    fn create_form_data(fields: &[(&str, String)]) -> FormData {
        let mut form_data = FormData::new();

        for (k, v) in fields {
            form_data.fields.insert(k.to_string(), v.to_string());
        }

        form_data        
    }

    #[test]
    fn test_cast_form_data_to_new_product() {
        let name = format!("product name");
        let description = format!("description for product");
        let url = format!("https://yahoo.com");
        let price = 123000.05;

        let form_data = create_form_data(&[
            ("name", name.clone()),
            ("price", price.to_string()),
        ]);

        let test_min_product = NewProduct {
            name: name.clone(),
            description: None,
            url: None,
            price: price.clone(),
            available: false
        };

        let new_min_product = cast_form_data_to_new_product(&form_data)
            .expect("Error casting");
        
        assert_eq!(new_min_product, test_min_product, "minimal form data casted to new product");

        let form_data_2 = create_form_data(&[
            ("name", name.clone()),
            ("description", description.clone()),
            ("url", url.clone()),
            ("price", price.to_string()),

        ]);

        let test_full_product = NewProduct {
            name,
            description: Some(description),
            url: Some(url),
            price,
            available: false
        };

        let new_full_product = cast_form_data_to_new_product(&form_data_2)
            .expect("Error casting");
        
        assert_eq!(new_full_product, test_full_product, "full form data casted to new product");
    }

    #[test]
    #[should_panic]
    fn test_cast_form_data_to_new_product_fail() {
        let name = format!("A failing product");

        let form_data = create_form_data(&[ ("name", name.clone())]);

        cast_form_data_to_new_product(&form_data).unwrap();
    }
}