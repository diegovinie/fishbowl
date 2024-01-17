use crate::api::resources::products::models::Product;
use crate::api::resources::users::models::User;
use diesel::result::Error;

pub trait DatabaseService: Send + Sync {
    fn user_repo(&self) -> Box<dyn UserRepo>;

    fn product_repo(&self) -> Box<dyn ProductRepo>;
}

pub trait UserRepo {
    fn find_user(&self, id: i32) -> Result<User, Error>;
}

pub trait ProductRepo {
    fn find_product(&self, id: i32) -> Result<Product, Error>;
}