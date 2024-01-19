use diesel::prelude::*;
use crate::db;
use crate::database::{contracts::ProductRepo, establish_connection};
use crate::api::utils::pagination::Paginate;
use crate::schema::products::table as products_table;
use super::models::{Product, NewProduct, ListedProduct};
use diesel::result::Error;

pub struct Repo;

impl ProductRepo for Repo {
    fn find_product(&self, id: i32) -> Result<Product, Error> {
        let conn = &mut establish_connection();

        products_table
            .find(id)
            .select(Product::as_select())
            .first(conn)
    }

    fn list_products(&self) -> Result<Vec<ListedProduct>, Error> {
        let conn = &mut establish_connection();

        products_table
            .select(ListedProduct::as_select())
            .load(conn)
    }

    fn list_products_paginate(&self, page: i64, per_page: i64) -> Result<(i64, Vec<ListedProduct>), Error> {
        let conn = &mut establish_connection();

        let results: Vec<(Product, i64)> = products_table.into_boxed()
            .paginate(page)
            .per_page(per_page)
            .get_results(conn)?;

        match results.first() {
            None => Ok((0, vec![])),
            Some((_, entries)) => Ok((
                *entries,
                results.into_iter().map(|(p, _)| ListedProduct::from(p)).collect())
            )
        }
    }

    fn insert_product(&self, new_product: NewProduct) -> Result<Product, Error> {
        let conn = &mut establish_connection();

        diesel::insert_into(products_table)
            .values(&new_product)
            .returning(Product::as_returning())
            .get_result(conn)
    }
}

pub fn find_product(id: i32) -> Result<Product, Error> {
    let connection = &mut db::establish_connection();

    products_table
        .find(id)
        .select(Product::as_select())
        .first(connection)
}

pub fn list_products() -> Result<Vec<ListedProduct>, Error> {
    let conn = &mut db::establish_connection();

    products_table
        .select(ListedProduct::as_select())
        .load(conn)
}

pub fn list_products_paginate(page: i64, per_page: i64) -> Result<(i64, Vec<ListedProduct>), Error> {
    let conn = &mut db::establish_connection();

    let results: Vec<(Product, i64)> = products_table.into_boxed()
        .paginate(page)
        .per_page(per_page)
        .get_results(conn)?;

    match results.first() {
        None => Ok((0, vec![])),
        Some((_, entries)) => Ok((
            *entries,
            results.into_iter().map(|(p, _)| ListedProduct::from(p)).collect())
        )
    }
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

pub fn insert_batch(products: Vec<NewProduct>) -> Result<usize, Error> {
    let conn = &mut db::establish_connection();

    diesel::insert_into(products_table)
        .values(products)
        .execute(conn)
}