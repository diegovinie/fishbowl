use salvo::prelude::*;
use fishbowl::home::home_controller;
use fishbowl::api;
use fishbowl::api::auth;
use dotenvy::dotenv;
use std::env;

#[tokio::main]
async fn main() {
    dotenv().ok();
    let domain = env::var("DOMAIN").expect("`DOMAIN` must be set");
    let port = env::var("PORT").expect("`PORT` must be set");

    tracing_subscriber::fmt().init();

    let router = Router::new()
        .hoop(auth::decode_token())
        .get(home_controller)
        .push(Router::with_path("files/<**path>").get(
            StaticDir::new([
                env::current_dir().unwrap().join("static")
                ])
            .auto_list(true)
        ))
        .push(Router::with_path("auth").post(auth::authenticate))
        .push(api::get_router());

    let acceptor = TcpListener::new(format!("{domain}:{port}")).bind().await;

    Server::new(acceptor).serve(router).await;
}
