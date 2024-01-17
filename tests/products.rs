use fishbowl;
use fishbowl::database::mocks::{TestDatabaseService, ServiceData};
use fishbowl::database::{ServiceInjector, InjectableServices};
use salvo::prelude::*;
use salvo::test::{ResponseExt, TestClient};
use fishbowl::api;
use api::resources::products::models::Product;
use serde::Deserialize;

static BASE_URL: &str = "http://localhost";

fn router(service_injector: ServiceInjector) -> Router {
    Router::new()
        .hoop(service_injector)
        .push(api::get_router())
}

#[derive(Deserialize)]
struct Response<T> {
    data: T,
}

fn prepare_target(service_data: ServiceData) -> Service {
    let services = InjectableServices {
        database: TestDatabaseService::new(service_data),
    };

    let service_injector = ServiceInjector::new(services);

    let router = router(service_injector);

     Service::new(router)
}

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
        .take_json::<Response<Product>>()
        .await
        .unwrap();

    assert_eq!(existing_product_res.data, product1);

    let not_found_product_status_code = TestClient::get(format!("{BASE_URL}/api/v1/products/2"))
        .send(&target)
        .await
        .status_code
        .unwrap();

    assert_eq!(not_found_product_status_code, StatusCode::NOT_FOUND);
}