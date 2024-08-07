use diesel::prelude::*;
use serde::{Serialize, Deserialize};
use salvo::http::form::FormData;
use crate::api::errors::{ApiError, ApiResult};
use crate::api::validations::{FormValidator, Validator};
use crate::schema::products;
use crate::models::Mergeable;

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, PartialOrd)]
#[derive(Queryable, Selectable, AsChangeset)]
#[diesel(table_name = crate::schema::products)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Product {
    pub id: i32,
    pub name: String,
    pub description: Option<String>,
    pub url: Option<String>,
    pub price: f32,
    pub available: bool,
}

#[derive(Debug, PartialEq, PartialOrd)]
#[derive(Insertable)]
#[diesel(table_name = products)]
pub struct NewProduct {
    pub name: String,
    pub description: Option<String>,
    pub url: Option<String>,
    pub available: bool,
    pub price: f32,
}

impl Mergeable for Product {
    fn merge(self, form_data: &FormData) -> ApiResult<Self> {
        let validator = FormValidator(form_data);
        let mut updatable = self.clone();

        if let Some(name) = validator.get("name") {
            if name.is_empty() {
                return Err(ApiError::BadRequestError(format!("`name` cannot be empty")));
            }

            updatable.name = name.to_string();
        }

        if let Some(_) = validator.get("description") {
            updatable.description = validator.optional_string("description")?;
        }

        if let Some(_) = validator.get("url") {
            updatable.url = validator.optional_string("url")?;
        }

        if let Some(_) = validator.get("available") {
            updatable.available = validator.boolean("available")?;
        }

        if let Some(_) = validator.get("price") {
            updatable.price = validator.float("price")?;
        }

        Ok(updatable)
    }
}

#[derive(Serialize, Deserialize, Debug, PartialEq, PartialOrd)]
#[derive(Queryable, Selectable)]
#[diesel(table_name = crate::schema::products)]
pub struct ListedProduct {
    pub id: i32,
    pub name: String,
    pub price: f32,
    pub available: bool,
}

impl From<Product> for ListedProduct {
    fn from(value: Product) -> Self {
        let Product { id, name, price, available, .. } = value;

        Self { id, name, price, available }
    }
}