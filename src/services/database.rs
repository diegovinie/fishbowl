pub mod contracts;

use diesel::prelude::*;
use diesel::pg::PgConnection;
use std::env;
use contracts::{DatabaseService, UserRepo, ProductRepo, WishlistRepo, WishRepo, SponsorRepo};
use crate::api::resources::products;
use crate::api::resources::users;
use crate::api::resources::wishlists;
use crate::api::resources::wishes;
use crate::api::resources::sponsors;
use crate::api::resources::followers;
use crate::api::auth;

pub fn establish_connection() -> PgConnection {
    let database_url = env::var("DATABASE_URL")
        .expect("DATABASE_URL must be set");

    PgConnection::establish(&database_url)
        .unwrap_or_else(|_| panic!("Error connecting to {}", database_url))
}

pub struct MainDatabase;

impl DatabaseService for MainDatabase {
    fn user_repo(&self) -> Box<dyn UserRepo> {
        Box::new(users::repo::Repo)
    }

    fn product_repo(&self) -> Box<dyn ProductRepo> {
        Box::new(products::repo::Repo)
    }

    fn wish_repo(&self) -> Box<dyn WishRepo> {
        Box::new(wishes::repo::Repo)
    }

    fn wishlist_repo(&self) -> Box<dyn WishlistRepo> {
        Box::new(wishlists::repo::Repo)
    }

    fn sponsor_repo(&self) -> Box<dyn SponsorRepo> {
        Box::new(sponsors::repo::Repo)
    }
    
    fn auth_repo(&self) -> Box<dyn contracts::AuthRepo> {
        Box::new(auth::repo::Repo)
    }

    fn follower_repo(&self) -> Box<dyn contracts::FollowerRepo> {
        Box::new(followers::repo::Repo)
    }
}
