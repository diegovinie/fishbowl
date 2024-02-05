pub mod schema;
pub mod api;
pub mod home;
pub mod cli;
pub mod services;

use salvo::prelude::*;
use api::auth;
use home::home_controller;
use salvo::catcher::Catcher;
use salvo::cors::Cors;
use salvo::http::Method;
use std::collections::HashMap;
use std::env;
use services::ServiceInjector;
pub use services::database as db;

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

struct ConfigParams(pub HashMap<String, String>);

impl ConfigParams {
    pub fn get(&self, key: &str) -> String {
        match self.0.get(key) {
            Some(value) => value.to_owned(),
            None => env::var(key).expect(&format!("`{}` must be set", key))
        }
    }
}

pub struct Config {
    domain: String,
    port: String,
    client_url: String,
    // params: ConfigParams,
}

impl Config {
    pub fn build() -> Self {
        let params = Self::params();
        let domain = params.get("DOMAIN");
        let port = params.get("PORT");
        let client_url = params.get("CLIENT_URL");

        Self {
            domain,
            port,
            client_url,
            // params,
        }
    }

    fn params() -> ConfigParams {
        let args = env::args();

        let params: HashMap<String, String> = args.filter(|arg| arg.starts_with("--"))
            .map(move |arg| arg.split("=")
                .map(|x| x.to_string())
                .collect::<Vec<String>>())
            .filter_map(|s| {
                match (s.get(0), s.get(1)) {
                    (None, _) => None,
                    (_, None) => None,
                    (Some(key), Some(val)) => {
                        Some((key.to_uppercase().replace("--", ""), String::from(val)))
                    }
                }
            })
            .collect();

        ConfigParams(params)
    }
}

impl Default for Config {
    fn default() -> Self {
        Self {
            domain: String::default(),
            port: String::default(),
            client_url: String::default(),
            // params: ConfigParams(HashMap::new()),
        }
    }
}

#[tokio::main]
pub async fn start_server(service_injector: ServiceInjector, config: &Config) {
    let Config { domain, port, client_url, .. } = config;

    let cors_handler = Cors::new()
        .allow_origin(vec![client_url.as_str()])
        .allow_methods(vec![Method::GET, Method::POST, Method::DELETE])
        .allow_headers(vec!["content-type", "authorization"])
        .into_handler();

    let router = Router::new()
        .hoop(service_injector)
        .hoop(cors_handler.clone())
        .hoop(auth::decode_bearer_token())
        .get(home_controller)
        .push(
            Router::with_path("files/<**path>")
                .get(StaticDir::new([env::current_dir().unwrap().join("static")]).auto_list(true)),
        )
        .push(auth::get_router())
        .push(api::get_router());

    let service = Service::new(router).catcher(Catcher::default().hoop(cors_handler));

    let acceptor = TcpListener::new(format!("{domain}:{port}")).bind().await;

    Server::new(acceptor).serve(service).await;
}

