use super::contracts::{DatabaseService, UserRepo, ProductRepo, SponsorRepo};
use crate::api::resources::products;
use crate::api::resources::users;
use crate::api::resources::sponsors;

pub struct DatabaseServiceImpl;

impl DatabaseService for DatabaseServiceImpl {
    fn user_repo(&self) -> Box<dyn UserRepo> {
        Box::new(users::repo::Repo)
    }

    fn product_repo(&self) -> Box<dyn ProductRepo> {
        Box::new(products::repo::Repo)
    }

    fn sponsor_repo(&self) -> Box<dyn SponsorRepo> {
        Box::new(sponsors::repo::Repo)
    }
}
