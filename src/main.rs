use salvo::prelude::*;
use fishbowl::api::{auth, products, wishlists};
use dotenvy::dotenv;
use std::env;

#[handler]
async fn hello(_req: &mut Request, _depot: &mut Depot, res: &mut Response) {
    res.render("Hi!");

}

#[tokio::main]
async fn main() {
    dotenv().ok();
    let domain = env::var("DOMAIN").expect("`DOMAIN` must be set");
    let port = env::var("PORT").expect("`PORT` must be set");

    tracing_subscriber::fmt().init();

    let router = Router::new()
        .hoop(auth::decode_token())
        .get(hello)
        .push(Router::with_path("auth").post(auth::authenticate))
        .push(products::get_router())
        .push(wishlists::get_router());

    let acceptor = TcpListener::new(format!("{domain}:{port}")).bind().await;

    Server::new(acceptor).serve(router).await;
}
