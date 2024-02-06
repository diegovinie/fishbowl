use super::CommandProcessor;
use crate::api::resources::users::models::User;

pub enum Target {
    Users,
    Help,
}

impl From<&String> for Target {
    fn from(value: &String) -> Self {
        match value.as_str() {
            "users" => Self::Users,
            "help" => Self::Help,
            other => panic!("Target: `{other}` not found"),
        }
    }
}

pub fn execute(target: Target, command_processor: &CommandProcessor) {
    match target {
        Target::Users => list_users(command_processor),
        Target::Help => print_list_help(),
    }
}

pub fn list_users(processor: &CommandProcessor) {
    let repo = processor.database.user_repo();

    match repo.list() {
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
}

pub fn print_list_help() {
    println!("{}", LIST_HELP_MESSAGE);
}

const LIST_HELP_MESSAGE: &str = r#"
    List command options:

    users       e.g. `cargo run -- list users`

    help        Show this screen
"#;