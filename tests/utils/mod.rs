pub mod test_product_repo;

use salvo::prelude::*;
use fishbowl::api;
use api::utils::pagination::Paginate;
use fishbowl::database::{ServiceInjector, InjectableServices};
use fishbowl::database::contracts;
use api::resources::products::models::Product;
use api::resources::users::models::User;
use test_product_repo::TestProductRepo;

pub static BASE_URL: &str = "http://localhost/api/v1";

#[derive(Clone)]
pub struct ServiceData {
    pub products: Vec<Product>,
    pub users: Vec<User>,
}

impl ServiceData {
    pub fn with_products(products: Vec<Product>) -> Self {
        let def = Self::default();

        Self { products, ..def }
    }
}

impl Default for ServiceData {
    fn default() -> Self {
        Self {
            products: vec![],
            users: vec![],
        }
    }
}

pub struct TestDatabaseService {
    pub data: ServiceData,
}

impl TestDatabaseService {
    pub fn new(data: ServiceData) -> Self {
        Self { data }
    }
}

impl contracts::DatabaseService for TestDatabaseService {
    fn user_repo(&self) -> Box<dyn contracts::UserRepo> {
        todo!()
    }

    fn product_repo(&self) -> Box<dyn contracts::ProductRepo> {
        Box::new(TestProductRepo::new(self.data.products.clone()))
    }
}


pub trait MockService<T> {
    fn new(data: Vec<T>) -> Self;

    fn data(&self) -> Vec<T>;
}

fn get_paginated_page<T: Clone>(items: &Vec<T>, page: i64, per_page: i64) -> &[T] {
    let mut pagination = items.clone().paginate(page);
    pagination = pagination.per_page(per_page);

    let start = pagination.offset as usize;
    let max = start + per_page as usize;
    let end = std::cmp::min(max, items.len());

    &items[start..end]
}

pub fn router(service_injector: ServiceInjector) -> Router {
    Router::new()
        .hoop(service_injector)
        .push(api::get_router())
}


pub fn prepare_target(service_data: ServiceData) -> Service {
    let services = InjectableServices {
        database: TestDatabaseService::new(service_data),
    };

    let service_injector = ServiceInjector::new(services);

    let router = router(service_injector);

     Service::new(router)
}