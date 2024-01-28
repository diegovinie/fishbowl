use std::{collections::HashMap, sync::{Arc, Mutex}};
use salvo::{test::{ResponseExt, TestClient}, hyper::StatusCode};
use fishbowl::api::responses::{CollectionResponse, ExecutionResponse};
use fishbowl::api::resources::users::models::User;
use crate::utils::{get_user_and_token, prepare_api_service, Reporter};

use super::utils::{prepare_target, ServiceData,  BASE_URL, get_admin_and_token};

fn test_users() -> HashMap<String, User> {
    let mut map = HashMap::new();

    map.insert("admin_user".to_string(), User {
        id: 1,
        name: "Sr admin".to_string(),
        email: "admin@dummy.test".to_string(),
        role: "ADMIN".to_string(),
        active: true,
    });

    map
}
#[tokio::test]
async fn list_users() {
    // -- setup 1
    let test_users = test_users();
    let admin_user = test_users.get("admin_user").unwrap();

    let service_data = ServiceData::with_users(vec![admin_user.clone()]);

    let target = prepare_target(service_data);

    let (_, auth_token) = get_admin_and_token();

    let bearer = format!("Bearer {auth_token}");

    // -- run 1

    let response = TestClient::get(format!("{BASE_URL}/admin/users"))
        .add_header("authorization", bearer, true)
        .send(&target)
        .await
        .take_json::<CollectionResponse<User>>()
        .await
        .unwrap();

    // -- assert 1
    assert_eq!(response.data, vec![admin_user.clone()], "the user list must be the same");

    // -- setup 2

    let (_, auth_token) = get_user_and_token();

    let bearer = format!("Bearer {auth_token}");

       // -- run 2

    let status_code = TestClient::get(format!("{BASE_URL}/admin/users"))
       .add_header("authorization", bearer, true)
       .send(&target)
       .await
       .status_code
       .unwrap();

   // -- assert 2

   assert_eq!(status_code, StatusCode::FORBIDDEN, "A not admin user gets forbidden");

    // -- run 3

    let status_code = TestClient::get(format!("{BASE_URL}/admin/users"))
        .send(&target)
        .await
        .status_code
        .unwrap();

      // -- assert 3

      assert_eq!(status_code, StatusCode::UNAUTHORIZED, "An unknown user get unauthorized");


}

#[tokio::test]
async fn populate_wishlists() {
    // -- setup

    let service_data = ServiceData::with_wishlists(vec![]);
    let reporter = Arc::new(Mutex::new(Reporter::new()));

    let target = prepare_api_service(service_data, reporter.clone());

    let (_, auth_token) = get_admin_and_token();

    let bearer = format!("Bearer {auth_token}");

    // -- run 1

    let response = &mut TestClient::post(format!("{BASE_URL}/admin/populate/wishlists"))
       .add_header("authorization", bearer, true)
       .send(&target)
       .await;

    let status_code = response.status_code.unwrap();

    let response_text: ExecutionResponse = response.take_json().await.unwrap();

    let calls = reporter.lock().expect("").get_fn_calls("wishlist_repo.insert_many");

    // -- assert 1

    assert_eq!(status_code, 202, "Status code must me 202");
    assert_eq!(calls, 1, "wishlist_repo.insert_many() should be called once");
    assert_eq!(response_text.message, "Total row affected: 10", "total of insertions must match");
}
