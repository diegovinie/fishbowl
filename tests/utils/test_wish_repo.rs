use std::sync::{Arc, Mutex};
use diesel::result::Error;
use fishbowl::api::resources::products::models::Product;
use fishbowl::api::resources::wishes::models::{Wish, NewWish, WishProduct};
use fishbowl::models::Composable;
use fishbowl::services::database::contracts;
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
    fn list_by_wishlist(&self, _id: i32) -> Result<Vec<WishProduct>, Error> {
        self.reporter.lock()
            .expect("Locking Reporter failed")
            .register_fn_call("wish_repo.list_by_wishlist");

        Ok(vec![])
    }
    
    fn insert(&self, new_wish: NewWish) -> Result<Wish, Error> {
        self.reporter.lock()
            .expect("Locking Reporter failed")
            .register_fn_call("wish_repo.insert");

        let NewWish { wishlist_id, product_id } = new_wish;

        Ok(Wish { id: 3, wishlist_id, product_id, pending: true })
    }

    fn find_one(&self, id: i32) -> Result<Wish, Error> {
        self.reporter.lock()
            .expect("Locking Reporter failed")
            .register_fn_call("wish_repo.find_one");

        let wish = self.data.iter().find(|w| w.id == id).ok_or(Error::NotFound)?;

        Ok(wish.clone())
    }
    
    fn find_one_expanded(&self, id: i32) -> Result<WishProduct, Error> {
        self.reporter.lock()
            .expect("")
            .register_fn_call("wish_repo.find_one_expanded");

        let wish = self.data.iter().find(|w| w.id == id).ok_or(Error::NotFound)?;

        let product = Product { id: 1, name: "".to_string(), description: None, url: None, price: 2000.0, available: true };
        let wish_product = WishProduct::compose(wish.clone(), product);
        Ok(wish_product)
    }

    fn delete(&self, id: i32) -> Result<usize, Error> {
        self.reporter.lock()
            .expect("")
            .register_fn_call("wish_repo.delete");

        Ok(1)
    }
}
