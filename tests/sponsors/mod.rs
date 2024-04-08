use std::sync::{Arc, Mutex};
use salvo::test::TestClient;
use crate::utils::{get_admin_and_token, prepare_api_service, Reporter, ServiceData, BASE_URL};

#[tokio::test]
async fn add_sponsor() {
    // setup

    let service_data = ServiceData::default();

    let reporter = Arc::new(Mutex::new(Reporter::new()));

    let target = prepare_api_service(service_data, reporter.clone());

    let (_, auth_token) = get_admin_and_token();

    let bearer = format!("Bearer {auth_token}");

    // run

    let fields = [
        ("user_id", "1"),
        ("wish_id", "1"),
        ("amount", "12000"),
        ("leader", "true"),
    ];

    let response = TestClient::post(format!("{BASE_URL}/sponsors"))
        .add_header("authorization", &bearer, true)
        .form(&fields)
        .send(&target)
        .await;

    // assert

    let locked_reporter = reporter.lock().unwrap();

    let insert_calls = locked_reporter.get_fn_calls("sponsor_repo.insert");

    drop(locked_reporter);

    let status_code = response.status_code.unwrap();

    assert_eq!(status_code, 200, "status code should be accepted 200");
    assert_eq!(insert_calls, 1, "sponsor_repo.insert() should be called once");
}