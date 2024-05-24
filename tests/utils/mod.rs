pub mod test_product_repo;
pub mod test_user_repo;
pub mod test_wishlist_repo;
pub mod test_wish_repo;
pub mod test_sponsor_repo;

use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use fishbowl::api::resources::sponsors::models::Sponsor;
use fishbowl::api::resources::wishes::models::Wish;
use salvo::prelude::*;
use fishbowl::api;
use api::utils::pagination::Paginate;
use fishbowl::api::auth;
use fishbowl::services::{ServiceInjector, InjectableServices};
use fishbowl::services::database::contracts;
use fishbowl::services::notifications::ConsoleNotifier;
use api::resources::products::models::Product;
use api::resources::wishlists::models::Wishlist;
use api::resources::users::models::User;
use api::auth::models::User as AuthUser;
use test_product_repo::TestProductRepo;
use test_user_repo::TestUserRepo;
use self::test_sponsor_repo::TestSponsorRepo;
use self::test_wish_repo::TestWishRepo;
use self::test_wishlist_repo::TestWishlistRepo;

pub static BASE_URL: &str = "http://localhost/api/v1";

#[derive(Clone)]
pub struct ServiceData {
    pub products: Vec<Product>,
    pub users: Vec<User>,
    pub wishlists: Vec<Wishlist>,
    pub wishes: Vec<Wish>,
    pub sponsors: Vec<Sponsor>,
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

    pub fn with_wishlists(wishlists: Vec<Wishlist>) -> Self {
        let def = Self::default();

        Self { wishlists, ..def }
    }

    pub fn products(self, products: Vec<Product>) -> Self {
        Self { products, ..self }
    }

    pub fn wishlists(self, wishlists: Vec<Wishlist>) -> Self {
        Self { wishlists, ..self }
    }

    pub fn wishes(self, wishes: Vec<Wish>) -> Self {
        Self { wishes, ..self }
    }

    pub fn sponsors(self, sponsors: Vec<Sponsor>) -> Self {
        Self { sponsors, ..self }
    }
}

impl Default for ServiceData {
    fn default() -> Self {
        Self {
            products: vec![],
            users: vec![],
            wishlists: vec![],
            wishes: vec![],
            sponsors: vec![],
        }
    }
}

#[derive(Clone)]
pub struct Reporter {
    fn_calls: HashMap<String, u8>,
}

impl Reporter {
    pub fn new() -> Self {
        Self {
            fn_calls: HashMap::new(),
        }
    }

    pub fn register_fn_call(&mut self, fn_name: &str) {
        self.fn_calls.insert(fn_name.to_string(), self.fn_calls.get(fn_name).unwrap_or(&0) + 1);
    }

    pub fn get_fn_calls(&self, fn_name: &str) -> u8 {
        *self.fn_calls.get(fn_name).unwrap_or(&0)
    }

    pub fn clear(&mut self) {
        self.fn_calls.clear();
    }
}

#[derive(Clone)]
pub struct TestDatabaseService {
    pub data: ServiceData,
    pub reporter: Arc<Mutex<Reporter>>,
}

impl TestDatabaseService {
    pub fn new(data: ServiceData) -> Self {
        Self {
            data,
            reporter: Arc::new(Mutex::new(Reporter::new())),
        }
    }

    pub fn with_reporter(data: ServiceData, reporter: Arc<Mutex<Reporter>>) -> Self {
        Self { data, reporter }
    }
}

impl contracts::DatabaseService for TestDatabaseService {
    fn user_repo(&self) -> Box<dyn contracts::UserRepo> {
        let repo = TestUserRepo::new(self.data.users.clone(), self.reporter.clone());

        Box::new(repo)
    }

    fn product_repo(&self) -> Box<dyn contracts::ProductRepo> {
        Box::new(TestProductRepo::new(self.data.products.clone(), self.reporter.clone()))
    }

    fn sponsor_repo(&self) -> Box<dyn contracts::SponsorRepo> {
        Box::new(TestSponsorRepo::new(self.data.sponsors.clone(), self.reporter.clone()))
    }

    fn wishlist_repo(&self) -> Box<dyn contracts::WishlistRepo> {
        Box::new(TestWishlistRepo::new(self.data.wishlists.clone(), self.reporter.clone()))
    }

    fn wish_repo(&self) -> Box<dyn contracts::WishRepo> {
        Box::new(TestWishRepo::new(self.data.wishes.clone(), self.reporter.clone()))
    }
}


pub trait MockService<T> {
    fn new(data: Vec<T>, reporter: Arc<Mutex<Reporter>>) -> Self;

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
        .hoop(auth::decode_bearer_token())
        .hoop(service_injector)
        .push(api::auth::get_router())
        .push(api::get_router())
}

pub fn prepare_target(service_data: ServiceData) -> Service {
    let services = InjectableServices {
        database: TestDatabaseService::new(service_data),
        notifier: ConsoleNotifier,
    };

    let service_injector = ServiceInjector::new(services);

    let router = router(service_injector);

     Service::new(router)
}

pub fn prepare_api_service(service_data: ServiceData, reporter: Arc<Mutex<Reporter>>) -> Service {
    let services = InjectableServices {
        database: TestDatabaseService::with_reporter(service_data, reporter),
        notifier: ConsoleNotifier,
    };

    let service_injector = ServiceInjector::new(services);

    let router = router(service_injector);

     Service::new(router)
}

pub fn get_user_and_token() -> (AuthUser, String) {
    let admin = AuthUser {
        id: 2,
        name: "Sr user".to_string(),
        email: "user@dummy.test".to_string(),
        role: "USER".to_string(),
        active: true,
        password: vec![],
    };

    let auth_token = auth::create_bearer_token(&admin).unwrap();

    (admin, auth_token)
}

pub fn get_admin_and_token() -> (AuthUser, String) {
    let admin = AuthUser {
        id: 1,
        name: "Sr admin".to_string(),
        email: "admin@dummy.test".to_string(),
        role: "ADMIN".to_string(),
        active: true,
        password: vec![],
    };

    let auth_token = auth::create_bearer_token(&admin).unwrap();

    (admin, auth_token)
}