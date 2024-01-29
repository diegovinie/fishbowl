use salvo::prelude::*;
use salvo::http::form::FormData;
use crate::api::errors::{ApiResult, ApiError};
use crate::api::utils::get_db;
use crate::api::utils::pagination::Pagination;
use crate::api::validations::{Validator, FormValidator};
use crate::api::responses as api_responses;
use crate::models::Updatable;
use super::models::NewProduct;

#[handler]
pub fn list_products(req: &mut Request, depot: &Depot, res: &mut Response) -> ApiResult<()> {
    let repo = get_db(depot)?.product_repo();

    match req.query::<i64>("per_page") {
        None => {
            let products = repo.list()?;

            api_responses::render_collection(res, products);
        },

        Some(per_page) => {
            let page = req.query::<i64>("page").unwrap_or(1);

            let (entries, products) = repo.list_paginated(page, per_page)?;

            api_responses::render_collection_paginated(res, products, Pagination::new(page, per_page, entries));
        }
    };

    Ok(())
}

#[handler]
pub async fn add_product(req: &mut Request, depot: &Depot, res: &mut Response) -> ApiResult<()> {
    let repo = get_db(depot)?.product_repo();

    let form_data = req.form_data().await?;

    let new_product = cast_form_data_to_new_product(form_data)?;

    let product = repo.insert(new_product)?;

    api_responses::render_resource_created(res, product);
    
    Ok(())
}

#[handler]
pub fn show_product(req: &Request, depot: &Depot, res: &mut Response) -> ApiResult<()> {
    let repo = get_db(depot)?.product_repo();
    
    let id = req.param::<i32>("id").ok_or(ApiError::FieldNotFound("id".to_string()))?;
    
    let product = repo.find_one(id)?;
    
    api_responses::render_resource(res, product);

    Ok(())
}

#[handler]
pub fn remove_product(req: &Request, depot: &Depot, res: &mut Response) -> ApiResult<()> {
    let repo = get_db(depot)?.product_repo();

    let id = req.param::<i32>("id").ok_or(ApiError::FieldNotFound("id".to_string()))?;

    let total_deleted = repo.delete(id)?;

    api_responses::render_db_execution(res, total_deleted);

    Ok(())
}

#[handler]
pub async fn update_product(req: &mut Request, depot: &Depot, res: &mut Response) -> ApiResult<()> {
    let repo = get_db(depot)?.product_repo();

    let id = req.param::<i32>("id").ok_or(ApiError::FieldNotFound("id".to_string()))?;

    let form_data = req.form_data().await?;

    let product = repo.find_one(id)?;

    let product_updated = product.merge(form_data);

    let updated_product = repo.update(&product_updated)?;

    api_responses::render_resource_updated(res, updated_product);

    Ok(())
}

fn cast_form_data_to_new_product(form_data: &FormData) -> Result<NewProduct, ApiError> {
    let validator = FormValidator(form_data);

    let name = validator.string("name")?;
    let description = validator.optional_string("description")?;
    let url = validator.optional_string("url")?;
    let price = validator.float("price")?;

    let new_product = NewProduct { name, description, url, price, available: false };

    Ok(new_product)
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