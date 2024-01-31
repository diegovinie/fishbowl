
use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use fishbowl::api::resources::products::models::Product;
use fishbowl::api::resources::wishlists::models::Wishlist;
use salvo::test::{ResponseExt, TestClient};
use fishbowl::api::resources::wishes::models::Wish;
use fishbowl::api::responses::ResourceResponse;
use crate::utils::get_admin_and_token;

use super::utils::{prepare_api_service, ServiceData, BASE_URL, Reporter};

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

fn test_wishes() -> HashMap<String, Wish> {
    let mut map = HashMap::new();
    map.insert("wish1".to_string(), Wish {
        id: 1,
        wishlist_id: 1,
        product_id: 1,
        pending: true,

    });

    map.insert("wish2".to_string(), Wish {
        id: 1,
        wishlist_id: 1,
        product_id: 2,
        pending: true,
    });

    map.insert("wish3".to_string(), Wish {
        id: 1,
        wishlist_id: 1,
        product_id: 3,
        pending: false,
    });

    map
}

#[tokio::test]
async fn show_wishlist() {
    // -- setup

    let test_products = test_products();

    let product1 = test_products.get("product1").unwrap();
    let product2 = test_products.get("product2").unwrap();
    let product3 = test_products.get("product3").unwrap();

    let products = vec![
        product1.clone(),
        product2.clone(),
        product3.clone(),
    ];

    let test_wishes = test_wishes();

    let wish1 = test_wishes.get("wish1").unwrap();
    let wish2 = test_wishes.get("wish2").unwrap();
    let wish3 = test_wishes.get("wish3").unwrap();

    let wishes = vec![
        wish1.clone(),
        wish2.clone(),
        wish3.clone(),
    ];

    let wishlist = Wishlist {
        id: 1,
        title: "Wishlist title".to_string(),
        description: Some("A meaningful description".to_string()),
        date: None,
        user_id: 1,
        published: true,
    };

    let service_data = ServiceData::default()
        .products(products)
        .wishes(wishes)
        .wishlists(vec![wishlist.clone()]);

    let reporter = Arc::new(Mutex::new(Reporter::new()));

    let target = prepare_api_service(service_data, reporter.clone());

    let (_, auth_token) = get_admin_and_token();

    let bearer = format!("Bearer {auth_token}");


    // -- run 1

    let response = &mut TestClient::get(format!("{BASE_URL}/wishlists/1?detailed=true"))
        .add_header("authorization", &bearer, true)
        .send(&target)
        .await;

    let status_code = response.status_code.unwrap();

    let parsed_response = response.take_json::<ResourceResponse<Wishlist>>()
        .await
        .unwrap();

    let locked_reporter = reporter.lock().unwrap();

    let wishlist_calls = locked_reporter.get_fn_calls("wishlist_repo.find_one");
    let wish_calls = locked_reporter.get_fn_calls("wish_repo.list_by_wishlist");

    drop(locked_reporter);

    // -- assert 1

    assert_eq!(status_code, 200, "status code should be 200");
    assert_eq!(wishlist_calls, 1, "wishlist_repo.find_one() should be called once");
    assert_eq!(wish_calls, 1, "wish_repo.list_by_wishlist() should be called once");
    assert_eq!(parsed_response.data, wishlist);

    // -- run 2

    let response = &mut TestClient::get(format!("{BASE_URL}/wishlists/2"))
        .add_header("authorization", &bearer, true)
        .send(&target)
        .await;

    let status_code = response.status_code.unwrap();

    // -- assert 2

    assert_eq!(status_code, 404, "status code should be 404 not found");
}