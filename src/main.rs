use std::{env, process};
use fishbowl::{Config, cli::{CommandOptions, process_command}};

fn main() {
    let args: Vec<String> = env::args().collect();

    let command_options = CommandOptions::build(&args).unwrap_or_else(|error| {
        println!("{}", error.message);
        process::exit(1);
    });

    let config = Config::build();

    process_command(command_options, config);
}
