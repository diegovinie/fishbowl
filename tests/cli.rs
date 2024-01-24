#![allow(dead_code)]

mod utils;

use std::sync::{Arc, Mutex};
use fishbowl;
use fishbowl::cli::{Command, PopulateTarget, CommandProcessor, ListTarget};
use utils::{ServiceData, TestDatabaseService, Reporter};

#[test]
fn populate_products() {
    // perform cli command
    let command = Command::Populate(PopulateTarget::Products);

    let reporter = Reporter::new();

    let service_data = ServiceData::default();

    let database = TestDatabaseService {
        data: service_data,
        reporter: Arc::new(Mutex::new(reporter)),
    };

    let command_processor = CommandProcessor {
        database: Box::new(database),
    };

    command_processor.process(command);


    // check for command output
}

#[test]
fn list_users() {
    let command = Command::List(ListTarget::Users);

    let users = vec![];

    let service_data = ServiceData::with_users(users);

    let database = TestDatabaseService {
        data: service_data,
        reporter: Arc::new(Mutex::new(Reporter::new())),
    };

    let command_processor = CommandProcessor {
        database: Box::new(database.clone()),
    };

    command_processor.process(command);

    let calls = database.reporter.lock()
        .expect("Locking Reporter failed")
        .get_fn_calls("user_repo.list");

    assert_eq!(calls, 1, "user_repo.list() should be called once");
}

#[test]
fn populate_users() {
    let command = Command::Populate(PopulateTarget::Users);

    let service_data = ServiceData::default();

    let reporter = Reporter::new();

    let database = TestDatabaseService {
        data: service_data,
        reporter: Arc::new(Mutex::new(reporter)),
    };

    let command_processor = CommandProcessor {
        database: Box::new(database.clone()),
    };

    command_processor.process(command);

    // let reports = database.get_reports();
}
