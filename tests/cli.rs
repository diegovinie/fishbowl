#![allow(dead_code)]

mod utils;

use fishbowl;
use fishbowl::cli::{Command, PopulateTarget, CommandProcessor};
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