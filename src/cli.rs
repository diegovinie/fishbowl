use std::process;
use super::{start_server, Config};
use crate::api::{admin::controllers::{parse_products_csv, parse_users_csv}, resources::users::models::User};
use crate::database::primary_service_injector;

pub enum Command {
    Serve,
    Populate(PopulateTarget),
    List(ListTarget),
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
                    None => {
                        print_populate_help();
                        process::exit(0);
                    },
                    Some(target) => Ok(Self::Populate(PopulateTarget::from(target)))
                },

                "list" => match args.get(2) {
                    None => {
                        print_list_help();
                        process::exit(0);
                    },
                    Some(target) => Ok(Self::List(ListTarget::from(target)))
                },

                other_action => Err(Error { message: format!("Action: `{other_action}` not found.")}),
            }
        }
    }
}

pub enum PopulateTarget {
    All,
    Products,
    Users,
    Help,
}

impl From<&String> for PopulateTarget {
    fn from(value: &String) -> Self {
        match value.as_str() {
            "all" => Self::All,
            "products" => Self::Products,
            "users" => Self::Users,
            "help" => Self::Help,
            other => panic!("Target: `{other}` not found"),
        }
    }
}

pub enum ListTarget {
    Users,
    Help,
}

impl From<&String> for ListTarget {
    fn from(value: &String) -> Self {
        match value.as_str() {
            "users" => Self::Users,
            "help" => Self::Help,
            other => panic!("Target: `{other}` not found"),
        }
    }
}

pub struct Error {
    pub message: String,
}

pub fn process_command(command: Command, config: Config) {
    match command {
        Command::Serve => start_server(primary_service_injector(), config),
        Command::Populate(target) => populate(target),
        Command::List(target) => list(target),
        Command::Help => print_help(),
    }
}

fn print_help() {
    println!("{}", HELP_MESSAGE);
}

fn print_populate_help() {
    println!("{}", POPULATE_HELP_MESSAGE);
}

fn print_list_help() {
    println!("{}", LIST_HELP_MESSAGE);
}

fn populate(target: PopulateTarget) {
    match target {
        PopulateTarget::All => {
            populate_users();
            populate_products();
        },
        PopulateTarget::Products => populate_products(),
        PopulateTarget::Users => populate_users(),
        PopulateTarget::Help => print_populate_help(),
    }
}

fn populate_products() {
    use crate::api::resources::products::repo::insert_batch;

    match parse_products_csv() {
        Err(error) => {
            println!("{error}");
        },

        Ok(products) => match insert_batch(products) {
            Err(error) => {
                println!("{error}");
            },
            Ok(total) => {
                println!("`Populate products` done. Total affected: {total}");
            }
        }
    }
}

fn populate_users() {
    use crate::api::resources::users::repo::insert_batch;

    match parse_users_csv() {
        Err(error) => {
            println!("{error}");
        },

        Ok(users) => match insert_batch(users) {
            Err(error) => {
                println!("{error}");
            },
            Ok(total) => {
                println!("`Populate users` done. Total affected: {total}");
            }
        }
    }
}

fn list(target: ListTarget) {
    use crate::api::resources::users::repo::list_all;

    match target {
        ListTarget::Users => match list_all() {
            Err(error) => {
                println!("{error}");
            },
            Ok(users) => {
                users.iter().for_each(|user| {
                    let User { id, name, email, role, active, .. } = user;

                    println!("{id:4}  {name:30}  {email:30}  {role:10}  {active:6}");
                });
            }
        }
        ListTarget::Help => print_list_help(),
    }
}

const HELP_MESSAGE: &str = r#"
    Commands:

    serve       Start http server: e.g. `cargo run -- serve`

    populate    Run `cargo run -- populate help` for more information

    list        Run `cargo run -- list help` for more information

    help        Show this screen

"#;

const POPULATE_HELP_MESSAGE: &str = r#"
    Populate command options:

    all         e.g. `cargo run -- populate all`

    users       e.g. `cargo run -- populate users`

    products    e.g. `cargo run -- populate products`

    help        Show this screen

"#;

const LIST_HELP_MESSAGE: &str = r#"
    List command options:

    help        Show this screen
"#;