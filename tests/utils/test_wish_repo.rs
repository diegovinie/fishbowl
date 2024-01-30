use std::sync::{Arc, Mutex};
use diesel::result::Error;
use fishbowl::api::resources::wishes::models::{Wish, NewWish, WishProduct};
use fishbowl::database::contracts;
use super::{MockService, Reporter};

pub struct TestWishRepo {
    pub data: Vec<Wish>,
    pub reporter: Arc<Mutex<Reporter>>,
}

impl MockService<Wish> for TestWishRepo {
    fn new(data: Vec<Wish>, reporter: Arc<Mutex<Reporter>>) -> Self {
        Self { data, reporter }
    }

    fn data(&self) -> Vec<Wish> {
        self.data.clone()
    }
}

impl contracts::WishRepo for TestWishRepo {
    fn list_by_wishlist(&self, id: i32) -> Result<Vec<WishProduct>, Error> {
        self.reporter.lock()
            .expect("Locking Reporter failed")
            .register_fn_call("wish_repo.list_by_wishlist");

        Ok(vec![])
    }
}
