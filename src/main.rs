use std::{env, process};
use dotenvy::dotenv;
use fishbowl::Config;
use fishbowl::services::database::MainDatabase;
use fishbowl::cli::{Command, CommandProcessor};

fn main() {
    dotenv().ok();

    let args: Vec<String> = env::args().collect();

    let command = Command::build(&args).unwrap_or_else(|error| {
        println!("{}", error.message);
        process::exit(1);
    });

    let config = Config::build();
    let database = Box::new(MainDatabase);
    let command_processor = CommandProcessor::new(database, config);

    command_processor.process(command);
}
