use std::collections::HashMap;
use salvo::test::{ResponseExt, TestClient};
use fishbowl::api::responses::CollectionResponse;
use fishbowl::api::resources::users::models::User;
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
    // -- setup
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
    assert_eq!(response.data, vec![admin_user.clone()]);
}