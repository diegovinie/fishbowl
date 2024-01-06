use dotenvy::dotenv;
use fishbowl::api;
use fishbowl::api::auth;
use fishbowl::home::home_controller;
use salvo::catcher::Catcher;
use salvo::cors::Cors;
use salvo::http::Method;
use salvo::prelude::*;
use std::env;

#[tokio::main]
async fn main() {
    dotenv().ok();
    let domain = env::var("DOMAIN").expect("`DOMAIN` must be set");
    let port = env::var("PORT").expect("`PORT` must be set");

    tracing_subscriber::fmt().init();

    let cors_handler = Cors::new()
        .allow_origin(vec!["http://localhost:5173"])
        .allow_methods(vec![Method::GET, Method::POST, Method::DELETE])
        .allow_headers(vec!["content-type", "authorization"])
        .into_handler();

    let router = Router::new()
        .hoop(cors_handler.clone())
        .hoop(auth::decode_token())
        .get(home_controller)
        .push(
            Router::with_path("files/<**path>")
                .get(StaticDir::new([env::current_dir().unwrap().join("static")]).auto_list(true)),
        )
        .push(Router::with_path("api/v1/auth").post(auth::authenticate))
        .push(api::get_router());

    let service = Service::new(router).catcher(Catcher::default().hoop(cors_handler));

    let acceptor = TcpListener::new(format!("{domain}:{port}")).bind().await;

    Server::new(acceptor).serve(service).await;
}
