use std::env;
use salvo::prelude::*;
use std::error::Error;
use serde::Deserialize;
use crate::api::{errors as api_errors, responses as api_responses};
use crate::api::resources::users::models::NewUser;
use crate::api::resources::products::models::NewProduct;

static USERS_CSV_FILE: &str = "data/users.csv";
static PRODUCTS_CSV_FILE: &str = "data/products.csv";

#[derive(Debug, Deserialize)]
struct UserBatch {
    name: String,
    email: String,
    password: String,
    active: bool,
}

impl Into<NewUser> for UserBatch {
    fn into(self) -> NewUser {
        let Self { name, email, password, active } = self;

        NewUser { name, email, password, active }
    }
}

#[derive(Debug, Deserialize)]
pub struct ProductBatch {
    pub name: String,
    pub price: f32,
    pub url: Option<String>,
    pub description: Option<String>,
}

impl Into<NewProduct> for ProductBatch {
    fn into(self) -> NewProduct {
        let Self { name, description, url, price } = self;

        NewProduct { name, price, url, description, available: true }
    }
}

#[handler]
pub fn populate_users(res: &mut Response) {
    use crate::api::resources::users::repo::insert_batch;

    match parse_users_csv() {
        Err(error) => api_errors::render_parse_field_error(res, error, "users.csv"),

        Ok(users) => match insert_batch(users) {
            Err(error) => api_errors::render_db_insert_error(res, error, "users"),

            Ok(total) => api_responses::render_db_execution(res, total)
        }
    }
}

#[handler]
pub fn populate_products(_req: &mut Request, res: &mut Response) {
    use crate::api::resources::products::repo::insert_batch;

    match parse_products_csv() {
        Err(error) => api_errors::render_parse_field_error(res, error, "products.csv"),

        Ok(products) => match insert_batch(products) {
            Err(error) => api_errors::render_db_insert_error(res, error, "products"),

            Ok(total) => api_responses::render_db_execution(res, total)
        }
    }
}

fn parse_users_csv() -> Result<Vec<NewUser>, Box<dyn Error>> {
    let current_dir = env::current_dir()?;
    let mut rdr = csv::Reader::from_path(current_dir.join(USERS_CSV_FILE))?;

    let mut users: Vec<NewUser> = vec![];

    for result in rdr.deserialize() {
        let user: UserBatch = result?;
        users.push(user.into())
    }

    Ok(users)
}

fn parse_products_csv() -> Result<Vec<NewProduct>, Box<dyn Error>> {
    let current_dir = env::current_dir()?;
    let mut rdr = csv::Reader::from_path(current_dir.join(PRODUCTS_CSV_FILE))?;

    let mut products: Vec<NewProduct> = vec![];

    for result in rdr.deserialize() {
        let product: ProductBatch = result?;
        products.push(product.into());
    }

    Ok(products)
}