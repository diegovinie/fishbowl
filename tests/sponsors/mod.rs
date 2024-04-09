use std::sync::{Arc, Mutex};
use fishbowl::api::resources::{sponsors::models::Sponsor, wishes::models::Wish};
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

#[tokio::test]
async fn list_sponsors_wish() {
    // setup

    let wishes = vec![
        Wish { id: 1, wishlist_id: 1, product_id: 1, pending: true },
    ];

    let sponsor1 = Sponsor {id: 1, leader: Some(true), amount: 56000.0, user_id: 1, wish_id: 1 };
    let sponsor2 = Sponsor {id: 2, leader: None, amount: 21000.5, user_id: 2, wish_id: 1 };
    let sponsor3 = Sponsor {id: 3, leader: None, amount: 100000.0, user_id: 1, wish_id: 2 };

    let sponsors = vec![
        sponsor1,
        sponsor2,
        sponsor3,
    ];

    let service_data = ServiceData::default()
        .wishes(wishes)
        .sponsors(sponsors);

    let reporter = Arc::new(Mutex::new(Reporter::new()));

    let target = prepare_api_service(service_data, reporter.clone());

    let (_, auth_token) = get_admin_and_token();

    let bearer = format!("Bearer {auth_token}");

    // run

    let response = TestClient::get(format!("{BASE_URL}/wishes/1/sponsors"))
        .add_header("authorization", &bearer, true)
        .send(&target)
        .await;

    // assert

    let status_code = response.status_code.unwrap();

    let locked_reporter = reporter.lock().unwrap();

    let list_calls = locked_reporter.get_fn_calls("sponsor_repo.list_by_wish");

    assert_eq!(status_code, 200, "status code should be ok 200");
    assert_eq!(list_calls, 1, "sponsor_repo.list_by_wish() should be called once");
}