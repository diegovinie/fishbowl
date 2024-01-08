use std::env;
use salvo::prelude::*;
use std::error::Error;
use serde::Deserialize;
use crate::api::{errors as api_errors, responses as api_responses};
use crate::api::resources::products::models::NewProduct;
use crate::api::resources::products::repo::insert_batch;

static PRODUCTS_CSV_FILE: &str = "data/products.csv";

#[derive(Debug, Deserialize)]
pub struct ProductBatch {
    pub name: String,
    pub price: f32,
    pub url: Option<String>,
    pub description: Option<String>,
}

#[handler]
pub fn populate_products(_req: &mut Request, res: &mut Response) {

    match parse_products_csv() {
        Err(error) => api_errors::render_parse_field_error(res, error, "products.csv"),

        Ok(products) => match insert_batch(products) {
            Err(error) => api_errors::render_db_insert_error(res, error, "products"),

            Ok(total) => api_responses::render_db_execution(res, total)
        }
    }
}

fn parse_products_csv() -> Result<Vec<NewProduct>, Box<dyn Error>> {
    let current_dir = env::current_dir()?;
    let mut rdr = csv::Reader::from_path(current_dir.join(PRODUCTS_CSV_FILE))?;

    let mut products: Vec<NewProduct> = vec![];

    for result in rdr.deserialize() {
        let product: ProductBatch = result?;
        products.push(NewProduct::from(product));
    }

    Ok(products)
}