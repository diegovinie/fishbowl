use super::contracts::{DatabaseService, ProductRepo};
use crate::api::resources::products::models::Product;
use crate::api::resources::users::models::User;
use diesel::result::Error;

pub struct ServiceData {
    pub products: Vec<Product>,
    pub users: Vec<User>,
}

pub struct TestDatabaseService {
    pub data: ServiceData,
}

impl TestDatabaseService {
    pub fn new(data: ServiceData) -> Self {
        Self { data }
    }
}

impl DatabaseService for TestDatabaseService {
    fn user_repo(&self) -> Box<dyn super::contracts::UserRepo> {
        todo!()
    }

    fn product_repo(&self) -> Box<dyn super::contracts::ProductRepo> {
        Box::new(TestProductRepo::new(self.data.products.clone()))
    }
}

pub trait HasId {
    fn id(&self) -> i32;
}


pub trait MockService<T> {
    fn new(data: Vec<T>) -> Self;

    fn data(&self) -> Vec<T>;
}

pub struct TestProductRepo {
  pub data: Vec<Product>
}

impl MockService<Product> for TestProductRepo {
    fn new(data: Vec<Product>) -> Self {
        Self { data }
    }

    fn data(&self) -> Vec<Product> {
        self.data.clone()
    }
}

impl ProductRepo for TestProductRepo {
    fn find_product(&self, id: i32) -> Result<Product, Error> {
        match self.data().into_iter().find(|item| item.id == id) {
            None => {
                Err(Error::NotFound)
            },
            
            Some(product) => Ok(product)
        }
    }
}

