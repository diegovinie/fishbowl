use super::contracts::{DatabaseService, UserRepo, ProductRepo, SponsorRepo};
use crate::api::resources::products;
use crate::api::resources::users;
use crate::api::resources::wishlists;
use crate::api::resources::wishes;
use crate::api::resources::sponsors;

pub struct DatabaseServiceImpl;

impl DatabaseService for DatabaseServiceImpl {
    fn user_repo(&self) -> Box<dyn UserRepo> {
        Box::new(users::repo::Repo)
    }

    fn product_repo(&self) -> Box<dyn ProductRepo> {
        Box::new(products::repo::Repo)
    }

    fn wish_repo(&self) -> Box<dyn super::contracts::WishRepo> {
        Box::new(wishes::repo::Repo)
    }

    fn wishlist_repo(&self) -> Box<dyn super::contracts::WishlistRepo> {
        Box::new(wishlists::repo::Repo)
    }

    fn sponsor_repo(&self) -> Box<dyn SponsorRepo> {
        Box::new(sponsors::repo::Repo)
    }
}
