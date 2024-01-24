use crate::api::resources::products::models::{Product, ListedProduct, NewProduct};
use crate::api::resources::users::models::{User, NewUser};
use diesel::result::Error;

pub trait DatabaseService: Send + Sync {
    fn user_repo(&self) -> Box<dyn UserRepo>;

    fn product_repo(&self) -> Box<dyn ProductRepo>;
}

pub trait UserRepo {
    fn list(&self) -> Result<Vec<User>, Error>;

    fn find_user(&self, id: i32) -> Result<User, Error>;

    fn insert_many(&self, users: Vec<NewUser>) ->Result<usize, Error>;
}

pub trait ProductRepo: Send + Sync {
    fn find_one(&self, id: i32) -> Result<Product, Error>;

    fn list(&self) -> Result<Vec<ListedProduct>, Error>;

    fn list_paginated(&self, page: i64, per_page: i64) -> Result<(i64, Vec<ListedProduct>), Error>;

    fn insert(&self, new_product: NewProduct) -> Result<Product, Error>;

    fn delete(&self, id: i32) -> Result<usize, Error>;

    fn update(&self, product: &Product) -> Result<Product, Error>;

    fn insert_many(&self, products: Vec<NewProduct>) -> Result<usize, Error>;
}