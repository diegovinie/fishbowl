
use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use fishbowl::api::resources::products::models::Product;
use fishbowl::api::resources::wishlists::models::Wishlist;
use salvo::test::{ResponseExt, TestClient};
use fishbowl::api::resources::wishes::models::{Wish, WishProduct};
use fishbowl::api::responses::ResourceResponse;
use crate::utils::{get_admin_and_token, get_user_and_token};

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

#[tokio::test]
async fn create_wishlist() {
    // setup
    
    let service_data = ServiceData::default();

    let reporter = Arc::new(Mutex::new(Reporter::new()));

    let target = prepare_api_service(service_data, reporter.clone());

    let (user, auth_token) = get_user_and_token();

    let bearer = format!("Bearer {auth_token}");

    let title = "One title";
    let date = "2024-02-24 17:00:00";
    
    // run 1

    let fields = [
        ("title", title),
        ("date", date),
    ];

    let mut response = TestClient::post(format!("{BASE_URL}/wishlists"))
        .add_header("authorization", &bearer, true)
        .form(&fields)
        .send(&target)
        .await;

    let status_code = response.status_code.unwrap();

    assert_eq!(status_code, 202, "status code should be accepted 202");

    let calls = reporter.lock()
        .expect("Failed")
        .get_fn_calls("wishlist_repo.insert");

    assert_eq!(calls, 1, "wishlist_repo.insert() should be called once");

    let wishlist = response.take_json::<ResourceResponse<Wishlist>>()
        .await
        .unwrap()
        .data;

    assert_eq!(wishlist.title, title, "title should be the same");
    assert_eq!(wishlist.user_id, user.id, "user_id should be authenticated user id");

    drop(reporter);
}

#[tokio::test]
async fn add_wish() {
    // -- setup

    let test_products = test_products();

    let product1 = test_products.get("product1").unwrap();

    let products = vec![
        product1.clone(),
    ];

    let test_wishes = test_wishes();

    let wish1 = test_wishes.get("wish1").unwrap();
    let wish2 = test_wishes.get("wish2").unwrap();

    let wishes = vec![
        wish1.clone(),
        wish2.clone(),
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
        .wishlists(vec![wishlist.clone()])
        .wishes(wishes);

    let reporter = Arc::new(Mutex::new(Reporter::new()));

    let target = prepare_api_service(service_data, reporter.clone());

    let (_user, auth_token) = get_admin_and_token();

    let bearer = format!("Bearer {auth_token}");

    // - run

    let fields = [
        ("wishlist_id", "1"),
        ("product_id", "1"),
    ];

    let response = TestClient::post(format!("{BASE_URL}/wishlists/1/wishes"))
        .add_header("authorization", &bearer, true)
        .form(&fields)
        .send(&target)
        .await;

    let status_code = response.status_code.unwrap();

    // -- assert

    let locked_reporter = reporter.lock().unwrap();

    let find_wishlist_calls = locked_reporter.get_fn_calls("wishlist_repo.find_one");
    let insert_wish_calls = locked_reporter.get_fn_calls("wish_repo.insert");

    drop(locked_reporter);

    assert_eq!(status_code, 202,  "status code should be 202");
    assert_eq!(find_wishlist_calls, 1, "wishlist_repo.find_one() should be called once");
    assert_eq!(insert_wish_calls, 1, "wish_repo.insert() should be called once");
}

#[tokio::test]
async fn show_wish() {
    let test_products = test_products();

    let product = test_products.get("product1").unwrap();

    let test_wishes = test_wishes();

    let wish = test_wishes.get("wish1").unwrap();

    let wishlist = Wishlist {
        id: 1,
        title: "Wishlist title".to_string(),
        description: Some("A meaningful description".to_string()),
        date: None,
        user_id: 2,
        published: true,
    };

    let service_data = ServiceData::default()
        .products(vec![product.clone()])
        .wishlists(vec![wishlist.clone()])
        .wishes(vec![wish.clone()]);

    let reporter = Arc::new(Mutex::new(Reporter::new()));

    let target = prepare_api_service(service_data, reporter.clone());

    let (_user, auth_token) = get_user_and_token();

    let bearer = format!("Bearer {auth_token}");

    // run 1

    let response1 = &mut TestClient::get(format!("{BASE_URL}/wishlists/1/wishes/1"))
        .add_header("authorization", &bearer, true)
        .send(&target)
        .await;

    let status_code = response1.status_code.unwrap();

    assert_eq!(status_code, 200, "status code should be 200");

    // ---

    let wish_product = response1.take_json::<ResourceResponse<WishProduct>>().await.unwrap();

    assert_eq!(wish_product.data.id, wish.id, "Wish id should be the same");

    // ---

    let mut locked_reporter = reporter.lock().unwrap();

    let fn_called = locked_reporter.get_fn_calls("wish_repo.find_one_expanded");

    assert_eq!(fn_called, 1, "wish_repo.find_one_expanded() should be called once");

    locked_reporter.clear();

    drop(locked_reporter);

    // run 2

    let (_, auth_admin_token) = get_admin_and_token();

    let response2 = &mut TestClient::get(format!("{BASE_URL}/wishlists/1/wishes/1?detailed=true"))
        .add_header("authorization", format!("Bearer {auth_admin_token}"), true)
        .send(&target)
        .await;

    let status_code = response2.status_code.unwrap();

    assert_eq!(status_code, 403, "status code should be 403 not allowed");

    // ---

    let mut locked_reporter = reporter.lock().unwrap();

    let fn_called = locked_reporter.get_fn_calls("wish_repo.find_one_expanded");

    assert_eq!(fn_called, 0, "wish_repo.find_one_expanded() shouldn't be called");

    locked_reporter.clear();

    drop(locked_reporter);

    // run 3

    let response3 = &mut TestClient::get(format!("{BASE_URL}/wishlists/1/wishes/1?detailed=true"))
        .add_header("authorization", &bearer, true)
        .send(&target)
        .await;

    let status_code = response3.status_code.unwrap();

    assert_eq!(status_code, 200, "status code should be 200");

    // ---

    let wish_product = response3.take_json::<ResourceResponse<WishProduct>>().await.unwrap();

    assert_eq!(wish_product.data.id, wish.id, "Wish id should be the same");

    // ---

    let mut locked_reporter = reporter.lock().unwrap();

    let fn_called = locked_reporter.get_fn_calls("wish_repo.find_one_expanded");

    assert_eq!(fn_called, 1, "wish_repo.find_one_expanded() should be called once");

    locked_reporter.clear();

    drop(locked_reporter);
}