use diesel::prelude::*;
use serde::{Serialize, Deserialize};
use salvo::http::form::FormData;
use crate::schema::products;
use crate::models::Updatable;

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

impl Updatable for Product {
    fn merge(self, form_data: &FormData) -> Self {
        Self {
            id: self.id,
            name: form_data.fields.get("name")
                .unwrap_or(&self.name)
                .to_string(),
            description: match form_data.fields.get("description") {
                None => self.description,
                Some(value) => Some(value.clone()),
            },
            url: match form_data.fields.get("url") {
                None => self.url,
                Some(value) => Some(value.clone()),
            },
            price: form_data.fields.get("price")
                .unwrap_or(&"".to_string())
                .to_owned()
                .parse()
                .unwrap_or(self.price),
            available: self.available,
        }
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