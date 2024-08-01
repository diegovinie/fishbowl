use crate::api::resources::products::models::{Product, ListedProduct, NewProduct};
use crate::api::resources::sponsors::models::{NewSponsor, Sponsor};
use crate::api::resources::users::models::{User, NewUser};
use crate::api::resources::wishes::models::{NewWish, Wish, WishProduct};
use crate::api::resources::wishlists::models::{Wishlist, NewWishlist};
use crate::api::auth;
use diesel::result::Error;

pub trait DatabaseService: Send + Sync {
    fn user_repo(&self) -> Box<dyn UserRepo>;

    fn product_repo(&self) -> Box<dyn ProductRepo>;

    fn wishlist_repo(&self) -> Box<dyn WishlistRepo>;

    fn wish_repo(&self) -> Box<dyn WishRepo>;

    fn sponsor_repo(&self) -> Box<dyn SponsorRepo>;

    fn auth_repo(&self) -> Box<dyn AuthRepo>;
}

pub trait UserRepo: Send + Sync {
    fn list(&self) -> Result<Vec<User>, Error>;

    fn find_user(&self, id: i32) -> Result<User, Error>;

    fn insert(&self, new_user: NewUser) -> Result<User, Error>;

    fn insert_many(&self, users: Vec<NewUser>) ->Result<usize, Error>;
}

pub trait ProductRepo: Send + Sync {
    fn find_one(&self, id: i32) -> Result<Product, Error>;

    fn list(&self) -> Result<Vec<ListedProduct>, Error>;

    fn list_paginated(&self, page: i64, per_page: i64) -> Result<(i64, Vec<ListedProduct>), Error>;

    fn insert(&self, new_product: NewProduct) -> Result<Product, Error>;

    fn delete(&self, id: i32) -> Result<usize, Error>;

    fn update(&self, product: &Product) -> Result<Product, Error>;

    fn insert_many(&self, products: Vec<NewProduct>) -> Result<usize, Error>;
}

pub trait WishlistRepo: Send + Sync {
    fn find_one(&self, id: i32) -> Result<Wishlist, Error>;

    fn insert(&self, new_wishlist: NewWishlist) -> Result<Wishlist, Error>;

    fn insert_many(&self, wishlists: Vec<NewWishlist>) -> Result<usize, Error>;

    fn update(&self, wishlist: &Wishlist) -> Result<Wishlist, Error>;

    fn delete(&self, id: i32) -> Result<usize, Error>;
}

pub trait WishRepo: Send + Sync {
    fn list_by_wishlist(&self, id: i32) -> Result<Vec<WishProduct>, Error>;

    fn insert(&self, new_wish: NewWish) -> Result<Wish, Error>;

    fn find_one(&self, id: i32) -> Result<Wish, Error>;

    fn find_one_expanded(&self, id: i32) -> Result<WishProduct, Error>;

    fn delete(&self, id: i32) -> Result<usize, Error>;
}

pub trait SponsorRepo: Send + Sync {
    fn insert(&self, new_sponsor: NewSponsor) -> Result<Sponsor, Error>;

    fn list_by_wish(&self, wish_id: i32) -> Result<Vec<Sponsor>, Error>;
}

pub trait AuthRepo: Send {
    fn validate(&self, email_candidate: &str, password_candidate: &str) -> Option<auth::models::User>;

    fn activate(&self, user_id: i32, user_email: &str) -> Result<usize, diesel::result::Error>;
}
