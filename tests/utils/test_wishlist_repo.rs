use std::sync::{Arc, Mutex};
use diesel::result::Error;
use fishbowl::api::resources::wishlists::models::{Wishlist, NewWishlist};
use fishbowl::database::contracts;
use super::{MockService, Reporter};

pub struct TestWishlistRepo {
    pub data: Vec<Wishlist>,
    pub reporter: Arc<Mutex<Reporter>>,
}

impl MockService<Wishlist> for TestWishlistRepo {
    fn new(data: Vec<Wishlist>, reporter: Arc<Mutex<Reporter>>) -> Self {
        Self { data, reporter }
    }

    fn data(&self) -> Vec<Wishlist> {
        self.data.clone()
    }
}

impl contracts::WishlistRepo for TestWishlistRepo {
    fn insert_many(&self, wishlists: Vec<NewWishlist>) -> Result<usize, Error> {
        self.reporter.lock()
            .expect("Locking Reporter failed")
            .register_fn_call("wishlist_repo.insert_many");

        Ok(wishlists.len())
    }
}
