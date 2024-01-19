use crate::api::resources::products::models::{Product, ListedProduct, NewProduct};
use crate::api::resources::users::models::User;
use diesel::result::Error;

pub trait DatabaseService: Send + Sync {
    fn user_repo(&self) -> Box<dyn UserRepo>;

    fn product_repo(&self) -> Box<dyn ProductRepo>;
}

pub trait UserRepo {
    fn find_user(&self, id: i32) -> Result<User, Error>;
}

pub trait ProductRepo: Send + Sync {
    fn find_product(&self, id: i32) -> Result<Product, Error>;

    fn list_products(&self) -> Result<Vec<ListedProduct>, Error>;

    fn list_products_paginate(&self, page: i64, per_page: i64) -> Result<(i64, Vec<ListedProduct>), Error>;

    fn insert_product(&self, new_product: NewProduct) -> Result<Product, Error>;

    fn delete_product(&self, id: i32) -> Result<usize, Error>;
}