
use salvo::prelude::*;
use salvo::test::{ResponseExt, TestClient};
use std::collections::HashMap;
use fishbowl::api::resources::products::models::{Product, ListedProduct};
use fishbowl::api::responses::{ResourceResponse, CollectionResponse, CollectionPaginatedResponse};
use super::utils::{prepare_target, ServiceData,  BASE_URL};

fn test_products() -> HashMap<String, Product> {
    let mut map = HashMap::new();
    map.insert("product1".to_string(), Product {
        id: 1,
        name: format!("product 1"),
        description: Some(format!("desc")),
        url: Some(format!("any url")),
        price: 34000.6,
        available: true

    });

    map.insert("product2".to_string(), Product {
        id: 2,
        name: format!("product 2"),
        description: Some(format!("desc for 2")),
        url: None,
        price: 120000.6,
        available: true
    });

    map.insert("product3".to_string(), Product {
        id: 3,
        name: format!("product 3"),
        description: None,
        url: Some(format!("any url")),
        price: 34000.0,
        available: false
    });

    map
}

#[tokio::test]
async fn find_product() {
    // -- setup

    let test_products = test_products();
    let product1 = test_products.get("product1").unwrap();
    let service_data = ServiceData::with_products(vec![product1.clone()]);
    let target = prepare_target(service_data);

    // -- run 1

    let existing_product_res = TestClient::get(format!("{BASE_URL}/products/1"))
        .send(&target)
        .await
        .take_json::<ResourceResponse<Product>>()
        .await
        .unwrap();

    // -- assert 1

    assert_eq!(existing_product_res.data, product1.clone(), "find product by id");

    // -- run 2

    let not_found_product_status_code = TestClient::get(format!("{BASE_URL}/products/2"))
        .send(&target)
        .await
        .status_code
        .unwrap();

    // -- assert 2

    assert_eq!(not_found_product_status_code, StatusCode::NOT_FOUND, "not found status code");

    // -- run 3

    let wrong_param_status_code = TestClient::get(format!("{BASE_URL}/products/letter"))
        .send(&target)
        .await
        .status_code
        .unwrap();

    // -- assert 3

    assert_eq!(wrong_param_status_code, StatusCode::BAD_REQUEST, "incorrect id param status code");
}

#[tokio::test]
async fn list_products() {
    // -- setup

    let test_products = test_products();
    let product1 = test_products.get("product1").unwrap();
    let product2 = test_products.get("product2").unwrap();
    let product3 = test_products.get("product3").unwrap();

    let service_data = ServiceData::with_products(vec![
        product1.clone(),
        product2.clone(),
        product3.clone(),
    ]);

    let target = prepare_target(service_data.clone());

    // -- run 1

    let product_list_res = TestClient::get(format!("{BASE_URL}/products"))
        .send(&target)
        .await
        .take_json::<CollectionResponse<ListedProduct>>()
        .await
        .unwrap();

    let products = product_list_res.data;
    let product2_candidate = products.iter().find(|p| p.id == product2.id);

    // -- assert 1

    assert_eq!(products.len(), service_data.products.len(), "length must be the same");
    assert_eq!(
        product2_candidate, Some(&ListedProduct::from(product2.clone())),
        "find a product by id",
    );

    // -- run 2

    let product_list_pag_res = TestClient::get(format!("{BASE_URL}/products?page=2&per_page=2"))
        .send(&target)
        .await
        .take_json::<CollectionPaginatedResponse<ListedProduct>>()
        .await
        .unwrap();

    let pagination = product_list_pag_res.pagination;
    let products = product_list_pag_res.data;
    let first_product = products.first();

    // -- assert 2

    assert_eq!(pagination.total_pages, 2, "pagination: total pages");
    assert_eq!(pagination.page, 2, "pagination: current page");
    assert_eq!(pagination.entries, 3, "pagination: entries");
    assert_eq!(first_product, Some(&ListedProduct::from(product3.clone())), "first product of page");
}

#[tokio::test]
async fn add_product() {
    // -- setup

    let test_products = test_products();
    let product1 = test_products.get("product1").unwrap();
    let service_data = ServiceData::default();

    let target = prepare_target(service_data.clone());

    let Product { name, description, url, price, ..} = product1.clone();

    let fields = [
        ("name", name.clone()),
        ("description", description.clone().unwrap()),
        ("url", url.clone().unwrap()),
        ("price", price.to_string()),
    ];

    // -- run

    let response = TestClient::post(format!("{BASE_URL}/products"))
        .form(&fields)
        .send(&target)
        .await
        .take_json::<ResourceResponse<Product>>()
        .await
        .unwrap();

    // -- assert

    assert_eq!(response.data, Product { id: 1_i32, name, description, url, price, available: false });

}

#[tokio::test]
async fn remove_product() {
    //-- setup

    let products = test_products();
    let product1 = products.get("product1").unwrap();
    let service_data = ServiceData::with_products(vec![product1.clone()]);

    let target = prepare_target(service_data.clone());

    // -- run 1
    let status_code = TestClient::delete(format!("{BASE_URL}/products/1"))
        .send(&target)
        .await
        .status_code
        .unwrap();

    // -- assert 1

    assert_eq!(status_code, StatusCode::ACCEPTED, "product deleted status code");

    // -- run 2

    let status_code = TestClient::delete(format!("{BASE_URL}/products/2"))
        .send(&target)
        .await
        .status_code
        .unwrap();

    // -- assert 2

    assert_eq!(status_code, StatusCode::NOT_FOUND, "no product found status code");
}

#[tokio::test]
async fn update_product() {
    // -- setup

    let products = test_products();
    let product3 = products.get("product3").unwrap();
    let service_data = ServiceData::with_products(vec![product3.clone()]);

    let target = prepare_target(service_data.clone());

    let description = "a new description";
    let price = 99000.0;

    let updated_product = Product {
        description: Some(description.to_string()),
        price,
        ..product3.clone()
    };

    let fields = [
        ("description", description),
        ("price", &price.to_string()),
    ];

    // -- run 1

    let response = TestClient::put(format!("{BASE_URL}/products/3"))
        .form(&fields)
        .send(&target)
        .await
        .take_json::<ResourceResponse<Product>>()
        .await
        .unwrap();

    // -- assert 1

    assert_eq!(response.data, updated_product, "product must have fields updated");

    // -- run 2

    let status_code = TestClient::put(format!("{BASE_URL}/products/1"))
        .form(&fields)
        .send(&target)
        .await
        .status_code
        .unwrap();

    // -- assert 2

    assert_eq!(status_code, StatusCode::NOT_FOUND, "when a product is not found");
}