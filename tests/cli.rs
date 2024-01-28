#![allow(dead_code)]

mod utils;

use std::sync::{Arc, Mutex};
use fishbowl::{self, Config};
use fishbowl::cli;
use cli::{Command, CommandProcessor};
use utils::{ServiceData, TestDatabaseService, Reporter};

#[test]
fn populate_products() {
    // setup

    let command = Command::Populate(cli::populate::Target::Products);

    let reporter = Arc::new(Mutex::new(Reporter::new()));

    let service_data = ServiceData::default();

    let database = TestDatabaseService::with_reporter(service_data, reporter.clone());

    let command_processor = CommandProcessor {
        database: Box::new(database),
        config: Config::default(),
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

    let command = Command::List(cli::list::Target::Users);

    let users = vec![];

    let service_data = ServiceData::with_users(users);

    let reporter = Arc::new(Mutex::new(Reporter::new()));

    let database = TestDatabaseService::with_reporter(service_data.clone(), reporter.clone());

    let command_processor = CommandProcessor {
        database: Box::new(database.clone()),
        config: Config::default(),
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

    let command = Command::Populate(cli::populate::Target::Users);

    let service_data = ServiceData::default();

    let reporter = Arc::new(Mutex::new(Reporter::new()));

    let database = TestDatabaseService::with_reporter(service_data.clone(), reporter.clone());

    let command_processor = CommandProcessor {
        database: Box::new(database.clone()),
        config: Config::default(),
    };

    // -- run 1

    command_processor.process(command);

    let calls = reporter.lock()
        .expect("Locking Reporter failed")
        .get_fn_calls("user_repo.insert_many");

    // -- assert 1

    assert_eq!(calls, 1, "user_repo.insert_many() should be called once");
}

#[test]
fn populate_wishlists() {
    // -- setup

    let command = Command::Populate(cli::populate::Target::Wishlists);

    let service_data = ServiceData::default();

    let reporter = Arc::new(Mutex::new(Reporter::new()));

    let database = TestDatabaseService::with_reporter(service_data.clone(), reporter.clone());

    let command_processor = CommandProcessor {
        database: Box::new(database.clone()),
        config: Config::default(),
    };

    // -- run 1

    command_processor.process(command);

    let calls = reporter.lock()
        .expect("Locking Reporter failed")
        .get_fn_calls("wishlist_repo.insert_many");

    // -- assert 1

    assert_eq!(calls, 1, "wishlist_repo.insert_many() should be called once");
}

#[test]
fn populate_all() {
    // -- setup

    let command = Command::Populate(cli::populate::Target::All);

    let service_data = ServiceData::default();

    let reporter = Arc::new(Mutex::new(Reporter::new()));

    let database = TestDatabaseService::with_reporter(service_data.clone(), reporter.clone());

    let command_processor = CommandProcessor {
        database: Box::new(database.clone()),
        config: Config::default(),
    };

    // -- run 1

    command_processor.process(command);

    let locked_reporter = reporter.lock()
        .expect("Locking Reporter failed");

    let product_calls = locked_reporter.get_fn_calls("product_repo.insert_many");
    let user_calls = locked_reporter.get_fn_calls("user_repo.insert_many");
    let wishlist_calls = locked_reporter.get_fn_calls("wishlist_repo.insert_many");

    // -- assert 1

    assert_eq!(product_calls, 1, "product_repo.insert_many() should be called once");
    assert_eq!(user_calls, 1, "user_repo.insert_many() should be called once");
    assert_eq!(wishlist_calls, 1, "wishlist_repo.insert_many() should be called once");
}
