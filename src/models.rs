use diesel::prelude::*;
use serde::Serialize;

#[derive(Serialize, Debug)]
#[derive(Queryable, Selectable)]
#[diesel(table_name = crate::schema::products)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Product {
    pub id: i32,
    pub name: String,
    pub description: String,
    pub price: f32,
    pub available: bool,
}

use crate::schema::products;

#[derive(Insertable)]
#[diesel(table_name = products)]
pub struct NewProduct<'a> {
    pub name: &'a str,
    pub description: &'a str,
    pub available: bool,
    pub price: f32,
}