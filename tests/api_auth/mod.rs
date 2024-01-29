
use std::sync::{Arc, Mutex};
use salvo::test::{ResponseExt, TestClient};
use fishbowl::api::resources::users::models::User;
use fishbowl::api::responses::ResourceResponse;
use super::utils::{prepare_api_service, ServiceData, BASE_URL, Reporter};

#[tokio::test]
async fn sighup() {
    // -- setup

    let service_data = ServiceData::default();
    let reporter = Arc::new(Mutex::new(Reporter::new()));
    let target = prepare_api_service(service_data, reporter.clone());

    let fields = [
        ("name", "Axel Rose"),
        ("email", "axel@dummy.test"),
        ("password", "patience"),
    ];

    // -- run 1

    let response = &mut TestClient::post(format!("{}/auth/signup", BASE_URL))
        .form(&fields)
        .send(&target)
        .await;

    let status_code = response.status_code.unwrap();

    let parsed_response = response.take_json::<ResourceResponse<User>>()
        .await
        .unwrap();

    let calls = reporter.lock()
        .unwrap()
        .get_fn_calls("user_repo.insert");

    // -- assert 1

    assert_eq!(status_code, 202, "status code should be 200");
    assert_eq!(calls, 1, "user_repo.insert() should be called once");
    assert_eq!(parsed_response.data.email, "axel@dummy.test", "email should match");
}