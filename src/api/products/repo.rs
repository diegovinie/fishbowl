use diesel::prelude::*;
use crate::db;
use crate::schema::products::table as products_table;
use super::models::Product;

pub fn find_product(id: i32) -> Result<Product, diesel::result::Error> {
    let connection = &mut db::establish_connection();
    
    products_table
        .find(id)
        .select(Product::as_select())
        .first(connection)
}