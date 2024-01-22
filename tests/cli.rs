#![allow(dead_code)]

mod utils;

use fishbowl;
use fishbowl::cli::{Command, PopulateTarget, CommandProcessor, ListTarget};
use utils::{ServiceData, TestDatabaseService};

#[test]
fn populate_products() {
    // perform cli command
    let command = Command::Populate(PopulateTarget::Products);


    let service_data = ServiceData::default();

    let database = TestDatabaseService {
        data: service_data,
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
    };

    let command_processor = CommandProcessor {
        database: Box::new(database),
    };

    command_processor.process(command);
}
