use diesel::prelude::*;
use diesel::result::Error;
use crate::api::resources::products::models::Product;
use crate::api::resources::users::models::User;
use crate::schema::products::table as products_table;
use super::establish_connection;
use super::contracts::{DatabaseService, UserRepo, ProductRepo};

pub struct DatabaseServiceImpl;

impl DatabaseService for DatabaseServiceImpl {
    fn user_repo(&self) -> Box<dyn UserRepo> {
        Box::new(UserRepoImpl)
    }

    fn product_repo(&self) -> Box<dyn ProductRepo> {
        Box::new(ProductRepoImpl)
    }
}

struct ProductRepoImpl;

impl ProductRepo for ProductRepoImpl {
    fn find_product(&self, id: i32) -> Result<Product, Error> {
        let connection = &mut establish_connection();

        products_table
            .find(id)
            .select(Product::as_select())
            .first(connection)
    }
}

pub struct UserRepoImpl;

impl UserRepo for UserRepoImpl {
    fn find_user(&self, id: i32) -> Result<User, Error> {
    use crate::schema::users::table as users_table;
    use diesel::prelude::*;


    let conn = &mut establish_connection();

    users_table
        .find(id)
        .select(User::as_select())
        .first(conn)
    }
}