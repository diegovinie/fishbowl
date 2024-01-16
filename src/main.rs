use std::{env, process};
use fishbowl::{Config, cli::{Command, process_command}};

fn main() {
    let args: Vec<String> = env::args().collect();

    let command = Command::build(&args).unwrap_or_else(|error| {
        println!("{}", error.message);
        process::exit(1);
    });

    let config = Config::build();

    process_command(command, config);
}
