use super::CommandProcessor;
use crate::api::admin::controllers::{parse_products_csv, parse_users_csv};


pub enum Target {
    All,
    Products,
    Users,
    Help,
}

impl From<&String> for Target {
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

pub fn execute(target: Target, processor: &CommandProcessor) {
    match target {
        Target::All => {
            populate_users(&processor);
            populate_products(&processor);
        },
        _target @ Target::Products => populate_products(&processor),
        _target @ Target::Users => populate_users(&processor),
        Target::Help => print_help(),
    }
}

pub fn populate_products(processor: &CommandProcessor) {
    let repo = processor.database.product_repo();

    match parse_products_csv() {
        Err(error) => {
            println!("{error}");
        },

        Ok(products) => match repo.insert_many(products) {
            Err(error) => {
                println!("{error}");
            },
            Ok(total) => {
                println!("`Populate products` done. Total affected: {total}");
            }
        }
    }
}


fn populate_users(processor: &CommandProcessor) {
    let repo = processor.database.user_repo();

    match parse_users_csv() {
        Err(error) => {
            println!("{error}");
        },

        Ok(users) => match repo.insert_many(users) {
            Err(error) => {
                println!("{error}");
            },
            Ok(total) => {
                println!("`Populate users` done. Total affected: {total}");
            }
        }
    }
}

pub fn print_help() {
    println!("{}", POPULATE_HELP_MESSAGE);
}

const POPULATE_HELP_MESSAGE: &str = r#"
    Populate command options:

    all         e.g. `cargo run -- populate all`

    users       e.g. `cargo run -- populate users`

    products    e.g. `cargo run -- populate products`

    help        Show this screen

"#;
