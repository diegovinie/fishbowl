use std::sync::{Arc, Mutex};
use diesel::result::Error;
use fishbowl::api::resources::wishlists::models::{Wishlist, NewWishlist};
use fishbowl::services::database::contracts;
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
    fn find_one(&self, id: i32) -> Result<Wishlist, Error> {
        self.reporter.lock()
        .expect("Locking Reporter failed")
        .register_fn_call("wishlist_repo.find_one");


        self.data().iter()
            .find(|w| w.id == id)
            .map(|w| w.clone())
            .ok_or(Error::NotFound)
    }

    fn insert(&self, new_wishlist: NewWishlist) -> Result<Wishlist, Error> {
        
        self.reporter.lock()
            .expect("Locking Reporter failed")
            .register_fn_call("wishlist_repo.insert");

        let NewWishlist { title, description, date, user_id, published } = new_wishlist;

        let wishlist = Wishlist { id: 1, title, description, user_id, date, published };

        Ok(wishlist)
    }

    fn insert_many(&self, wishlists: Vec<NewWishlist>) -> Result<usize, Error> {
        self.reporter.lock()
            .expect("Locking Reporter failed")
            .register_fn_call("wishlist_repo.insert_many");

        Ok(wishlists.len())
    }
        
    fn update(&self, _wishlist: &Wishlist) -> Result<Wishlist, Error> {
        todo!()
    }
    
    fn delete(&self, _id: i32) -> Result<usize, Error> {
        self.reporter.lock()
            .expect("Locking Reporter failed")
            .register_fn_call("wishlist_repo.delete");

        Ok(1)
    }
}
