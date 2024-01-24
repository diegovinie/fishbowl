#![allow(dead_code)]

mod utils;

use std::sync::{Arc, Mutex};
use fishbowl;
use fishbowl::cli::{Command, PopulateTarget, CommandProcessor, ListTarget};
use utils::{ServiceData, TestDatabaseService, Reporter};

#[test]
fn populate_products() {
    // setup

    let command = Command::Populate(PopulateTarget::Products);

    let reporter = Arc::new(Mutex::new(Reporter::new()));

    let service_data = ServiceData::default();

    let database = TestDatabaseService::with_reporter(service_data, reporter.clone());

    let command_processor = CommandProcessor {
        database: Box::new(database),
    };

    // -- run 1

    command_processor.process(command);

    let calls = reporter.lock()
        .expect("Locking Reporter failed")
        .get_fn_calls("product_repo.insert_many");

    // -- assert 1
    
    assert_eq!(calls, 1, "product_repo.insert_many should be called once");
}

#[test]
fn list_users() {
    // -- setup

    let command = Command::List(ListTarget::Users);

    let users = vec![];

    let service_data = ServiceData::with_users(users);

    let reporter = Arc::new(Mutex::new(Reporter::new()));

    let database = TestDatabaseService::with_reporter(service_data.clone(), reporter.clone());

    let command_processor = CommandProcessor {
        database: Box::new(database.clone()),
    };

    // -- run 1

    command_processor.process(command);

    let calls = reporter.lock()
        .expect("Locking Reporter failed")
        .get_fn_calls("user_repo.list");

    // -- assert 1

    assert_eq!(calls, 1, "user_repo.list() should be called once");
}

#[test]
fn populate_users() {
    // -- setup

    let command = Command::Populate(PopulateTarget::Users);

    let service_data = ServiceData::default();

    let reporter = Arc::new(Mutex::new(Reporter::new()));

    let database = TestDatabaseService::with_reporter(service_data.clone(), reporter.clone());

    let command_processor = CommandProcessor {
        database: Box::new(database.clone()),
    };

    // -- run 1

    command_processor.process(command);

    let calls = reporter.lock()
        .expect("Locking Reporter failed")
        .get_fn_calls("user_repo.insert_many");

    // -- assert 1

    assert_eq!(calls, 1, "user_repo.insert_many() should be called once");
}
