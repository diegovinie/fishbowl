use super::{start_server, Config};

pub enum Command {
    Serve,
    Help,
}

pub struct Error {
    pub message: String,
}

pub struct CommandOptions {
    pub command: Command,
}

impl CommandOptions {
    pub fn build(args: &[String]) -> Result<Self, Error> {
        match args.get(1) {
            None => Ok(CommandOptions{ command: Command::Help }),

            Some(action) => match action.as_str() {
                "serve" => Ok(Self {
                    command: Command::Serve
                }),

                "help" => Ok(Self {
                    command: Command::Help
                }),

                other_action => Err(Error { message: format!("Action: `{other_action}` not found.")}),
            }
        }
    }
}

pub fn print_help() {
    println!("{}", HELP_MESSAGE);
}

pub fn process_command(command_options: CommandOptions, config: Config) {
    match command_options.command {
        Command::Serve => start_server(config),
        Command::Help => print_help(),
    }
}

const HELP_MESSAGE: &str = r#"
    Commands:

    serve       Start http server: e.g. `cargo run -- serve`

    help        Show this screen

"#;