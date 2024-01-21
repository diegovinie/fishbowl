pub mod test_product_repo;
pub mod test_user_repo;

use salvo::prelude::*;
use fishbowl::api;
use api::utils::pagination::Paginate;
// use api::auth;
use fishbowl::api::auth;
use fishbowl::database::{ServiceInjector, InjectableServices};
use fishbowl::database::contracts;
use api::resources::products::models::Product;
use api::resources::users::models::User;
use api::auth::models::User as AuthUser;
use test_product_repo::TestProductRepo;
use test_user_repo::TestUserRepo;

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

    pub fn with_users(users: Vec<User>) -> Self {
        let def = Self::default();

        Self { users, ..def }
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
        Box::new(TestUserRepo::new(self.data.users.clone()))
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
        .hoop(auth::decode_token())
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

pub fn get_admin_and_token() -> (AuthUser, String) {
    let admin = AuthUser { 
        id: 1,
        name: "Sr admin".to_string(),
        email: "admin@dummy.test".to_string(),
        role: "ADMIN".to_string(),
        active: true,
        password: "".to_string(),
    };

    let auth_token = auth::create_token(admin.clone()).unwrap();

    (admin, auth_token)
}