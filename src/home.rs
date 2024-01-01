use salvo::hyper::HeaderMap;
use salvo::prelude::*;
use std::env;

#[handler]
pub async fn home_controller(res: &mut Response) {
    let file = env::current_dir().unwrap()
        .join("static")
        .join("index.html");

    let req_headers = HeaderMap::new();

    res.send_file(file, &req_headers).await;
}
