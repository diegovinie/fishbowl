use std::sync::Arc;
use salvo::prelude::*;
use std::error::Error;
use serde::Deserialize;
use chrono::NaiveDateTime;
use crate::api::errors::{ApiResult, ApiError};
use crate::api::resources::wishlists::models::NewWishlist;
use crate::api::utils::{hash_password, parse_csv};
use crate::api::{errors as api_errors, responses as api_responses, utils};
use crate::api::resources::users::models::NewUser;
use crate::api::resources::products::models::NewProduct;
use crate::database::contracts::DatabaseService;
use crate::api::utils::formatters::optional_date;

static USERS_CSV_FILE: &str = "data/users.csv";
static PRODUCTS_CSV_FILE: &str = "data/products.csv";
static WISHLISTS_CSV_FILE: &str = "data/wishlists.csv";

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

        NewUser { name, email, password: hash_password(&password), active }
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
#[derive(Debug, Deserialize)]
pub struct WishlistBatch {
    pub title: String,
    pub description: Option<String>,
    #[serde(with="optional_date")]
    pub date: Option<NaiveDateTime>,
    pub user_id: i32,
    published: bool,
}

impl Into<NewWishlist> for WishlistBatch {
    fn into(self) -> NewWishlist {
        let Self { title, description, date, user_id, published } = self;

        NewWishlist { title, description, date, user_id, published }
    }
}

#[handler]
pub fn check_admin_role(depot: &Depot, res: &mut Response) {
    if !utils::admin(depot) {
        return api_errors::render_unauthorized(res);
    }
}

#[handler]
pub fn list_users(_req: &Request, depot: &Depot, res: &mut Response) -> ApiResult<()> {
    let repo = get_db(depot)?.user_repo();

    let users = repo.list()?;

    api_responses::render_collection(res, users);

    Ok(())
}

#[handler]
pub fn populate_users(depot: &Depot, res: &mut Response) ->ApiResult<()> {
    let repo = get_db(depot)?.user_repo();

    match parse_csv::<UserBatch, NewUser>(USERS_CSV_FILE) {
        Err(error) => api_errors::render_parse_field_error(res, error, "users.csv"),

        Ok(users) => match repo.insert_many(users) {
            Err(error) => api_errors::render_db_insert_error(res, error, "users"),

            Ok(total) => api_responses::render_db_execution(res, total)
        }
    };

    Ok(())
}

#[handler]
pub fn populate_products(_req: &mut Request, depot: &Depot, res: &mut Response) {
    let repo = get_db(depot).unwrap().product_repo();

    match parse_products_csv() {
        Err(error) => api_errors::render_parse_field_error(res, error, "products.csv"),

        Ok(products) => match repo.insert_many(products) {
            Err(error) => api_errors::render_db_insert_error(res, error, "products"),

            Ok(total) => api_responses::render_db_execution(res, total)
        }
    }
}

#[handler]
pub fn populate_wishlists(depot: &Depot, res: &mut Response) -> ApiResult<()> {
    let repo = get_db(depot).unwrap().wishlist_repo();

    let wishlists = parse_wishlist_csv()
        .map_err(|e| ApiError::Deserializer(format!("{}", e)))?;

    let total = repo.insert_many(wishlists)?;

    api_responses::render_db_execution(res, total);

    Ok(())
}

pub fn parse_users_csv() -> Result<Vec<NewUser>, Box<dyn Error>> {
    parse_csv::<UserBatch, NewUser>(USERS_CSV_FILE)
}

pub fn parse_products_csv() -> Result<Vec<NewProduct>, Box<dyn Error>> {
    parse_csv::<ProductBatch, NewProduct>(PRODUCTS_CSV_FILE)
}

pub fn parse_wishlist_csv() -> Result<Vec<NewWishlist>, Box<dyn Error>> {
    parse_csv::<WishlistBatch, NewWishlist>(WISHLISTS_CSV_FILE)
}

fn get_db(depot: &Depot) -> ApiResult<&Arc<dyn DatabaseService>> {
    use crate::api::errors::InjectionError;

    let service = depot.obtain::<Arc<dyn DatabaseService>>()
        .map_err(|_| ApiError::Injection(InjectionError))?;

    Ok(service)
}