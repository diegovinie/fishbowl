pub mod schema;
pub mod api;
pub mod home;
pub mod cli;
pub mod database;
pub use database as db;

use salvo::prelude::*;
use api::auth;
use home::home_controller;
use salvo::catcher::Catcher;
use salvo::cors::Cors;
use salvo::http::Method;
use std::env;
use database::ServiceInjector;

pub mod models {
    use salvo::http::form::FormData;
    use serde::{Deserialize, Serialize};

    pub trait Updatable {
        fn merge(self, form_data: &FormData) -> Self;
    }

    pub trait Composable<T, G> {
        fn compose(tree: T, branch: G) -> Self;
    }


    #[derive(Deserialize, Serialize, Debug, Copy, Clone)]
    pub enum Role {
        Admin,
        User,
    }

    impl<'a> From<&'a str> for Role {
        fn from(value: &'a str) -> Self {
            match value {
                "ADMIN" => Role::Admin,
                _other => Role::User,
            }
        }
    }
}

pub struct Config {
    domain: String,
    port: String,
    client_url: String,
}

impl Config {
    pub fn build() -> Self {
        let domain = env::var("DOMAIN").expect("`DOMAIN` must be set");
        let port = env::var("PORT").expect("`PORT` must be set");
        let client_url = env::var("CLIENT_URL").expect("`CLIENT_URL` must be set");

        Self {
            domain,
            port,
            client_url,
        }
    }
}

#[tokio::main]
pub async fn start_server(service_injector: ServiceInjector, config: Config) {
    let Config { domain, port, client_url } = config;

    let cors_handler = Cors::new()
        .allow_origin(vec![client_url.as_str()])
        .allow_methods(vec![Method::GET, Method::POST, Method::DELETE])
        .allow_headers(vec!["content-type", "authorization"])
        .into_handler();

    let router = Router::new()
        .hoop(service_injector)
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

