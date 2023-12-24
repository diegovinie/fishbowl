use diesel::prelude::*;
use crate::db;
use crate::schema::products::table as products_table;
use super::models::{Product, NewProduct};
use diesel::result::Error;

pub fn find_product(id: i32) -> Result<Product, Error> {
    let connection = &mut db::establish_connection();
    
    products_table
        .find(id)
        .select(Product::as_select())
        .first(connection)
}

pub fn list_products() -> Result<Vec<Product>, Error> {
    let conn = &mut db::establish_connection();
    
    products_table
        .select(Product::as_select())
        .load(conn)
}

pub fn insert_product(new_product: NewProduct) -> Result<Product, Error> {
    let conn = &mut db::establish_connection();

    diesel::insert_into(products_table)
        .values(&new_product)
        .returning(Product::as_returning())
        .get_result(conn)
}

pub fn update_product(product: &Product) -> Result<Product, Error> {
    let conn = &mut db::establish_connection();

    diesel::update(products_table.find(product.id))
        .set(product)
        .get_result(conn)
}

pub fn delete_product(id: i32) -> Result<usize, Error> {
    let conn = &mut db::establish_connection();

    diesel::delete(products_table.find(id))
        .execute(conn)
}