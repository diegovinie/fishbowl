pub mod populate;
pub mod list;

use super::{start_server, Config};
use crate::database::contracts::DatabaseService;
use crate::database::primary_service_injector;

pub enum Command {
    Serve,
    Populate(populate::Target),
    List(list::Target),
    Hash(String),
    Help,
}

impl Command {
    pub fn build(args: &[String]) -> Result<Self, Error> {
        match args.get(1) {
            None => Ok(Self::Help),

            Some(action) => match action.as_str() {
                "serve" => Ok(Self::Serve),

                "help" => Ok(Self ::Help),

                "populate" => match args.get(2) {
                    None => Ok(Self::Populate(populate::Target::Help)),
                    Some(target) => Ok(Self::Populate(populate::Target::from(target)))
                },

                "list" => match args.get(2) {
                    None => Ok(Self::List(list::Target::Help)),
                    Some(target) => Ok(Self::List(list::Target::from(target)))
                },

                "hash" => match args.get(2) {
                    None => todo!("Create help for hash"),
                    Some(target) => Ok(Self::Hash(target.to_string()))
                }

                other_action => Err(Error { message: format!("Action: `{other_action}` not found.")}),
            }
        }
    }
}


pub struct Error {
    pub message: String,
}

pub struct CommandProcessor {
    pub database: Box<dyn DatabaseService>,
    pub config: Config,
}

impl CommandProcessor {
    pub fn new(database: Box<dyn DatabaseService>, config: Config) -> Self {
        Self {
            database,
            config,
        }
    }

    pub fn  process(&self, command: Command) {
        match command {
            Command::Serve => start_server(primary_service_injector(), &self.config),
            Command::Populate(target) => populate::execute(target, self),
            Command::List(target) => list::execute(target, self),
            Command::Hash(target) => misc::hash(&target),
            Command::Help => print_help(),
        }
    }
}

pub mod misc {
    use crate::api::utils::hash_password;

    pub fn hash(text: &str) {
        let hashed = hash_password(&text);

        print!("{:x?}", hashed);
    }
}

fn print_help() {
    println!("{}", HELP_MESSAGE);
}


const HELP_MESSAGE: &str = r#"
    Commands:

    serve       Start http server: e.g. `cargo run -- serve`

    populate    Run `cargo run -- populate help` for more information

    list        Run `cargo run -- list help` for more information

    help        Show this screen

"#;
