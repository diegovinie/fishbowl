use fishbowl::{start_server, Config};

fn main() {
    let config = Config::build();

    start_server(config)
}
