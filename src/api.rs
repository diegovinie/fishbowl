pub mod auth;
pub mod products;

#[derive(Debug)]
pub enum Error<'a> {
    FieldNotFound(&'a str),
    ParseFloatErr(&'a str),
}

pub mod utils {
    use salvo::prelude::*;
    use std::str::FromStr;

    pub fn get_req_param<T: FromStr>(req: &mut Request, param: &str) -> Result<T, T::Err> {
        req.params()
            .get(param)
            .cloned()
            .unwrap_or_default()
            .parse()
    }
}