pub mod products;

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