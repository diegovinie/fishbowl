use salvo::prelude::*;
use diesel::result::Error;
use fishbowl::api;
use fishbowl::database::{ServiceInjector, InjectableServices};
use fishbowl::database::contracts;
use api::resources::products::models::Product;
use api::resources::users::models::User;


static BASE_URL: &str = "http://localhost";


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

impl contracts::ProductRepo for TestProductRepo {
    fn find_product(&self, id: i32) -> Result<Product, Error> {
        match self.data().into_iter().find(|item| item.id == id) {
            None => {
                Err(Error::NotFound)
            },

            Some(product) => Ok(product)
        }
    }
}


fn router(service_injector: ServiceInjector) -> Router {
    Router::new()
        .hoop(service_injector)
        .push(api::get_router())
}


fn prepare_target(service_data: ServiceData) -> Service {
    let services = InjectableServices {
        database: TestDatabaseService::new(service_data),
    };

    let service_injector = ServiceInjector::new(services);

    let router = router(service_injector);

     Service::new(router)
}

pub mod products {
    use salvo::prelude::*;
    use salvo::test::{ResponseExt, TestClient};
    use fishbowl::api::resources::products::models::Product;
    use fishbowl::api::responses::ResourceResponse;
    use super::ServiceData;
    use super::{prepare_target,  BASE_URL};

    #[tokio::test]
    async fn find_product() {
        let product1 = Product {
            id: 1,
            name: format!("product"),
            description: Some(format!("desc")),
            url: Some(format!("any url")),
            price: 34000.6,
            available: true
        };

        let service_data = ServiceData {
            users: vec![],
            products: vec![product1.clone()],
        };

        let target = prepare_target(service_data);

        let existing_product_res = TestClient::get(format!("{BASE_URL}/api/v1/products/1"))
            .send(&target)
            .await
            .take_json::<ResourceResponse<Product>>()
            .await
            .unwrap();

        assert_eq!(existing_product_res.data, product1);

        let not_found_product_status_code = TestClient::get(format!("{BASE_URL}/api/v1/products/2"))
            .send(&target)
            .await
            .status_code
            .unwrap();

        assert_eq!(not_found_product_status_code, StatusCode::NOT_FOUND);

        let wrong_param_status_code = TestClient::get(format!("{BASE_URL}/api/v1/products/letter"))
            .send(&target)
            .await
            .status_code
            .unwrap();

        assert_eq!(wrong_param_status_code, StatusCode::BAD_REQUEST);
    }
}