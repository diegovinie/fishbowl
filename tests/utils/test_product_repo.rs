use std::sync::{Arc, Mutex};
use diesel::result::Error;
use fishbowl::api::resources::products::models::{Product, ListedProduct, NewProduct};
use fishbowl::database::contracts;
use super::{MockService, get_paginated_page, Reporter};

pub struct TestProductRepo {
    pub data: Vec<Product>,
    pub reporter: Arc<Mutex<Reporter>>,
}

impl MockService<Product> for TestProductRepo {
    fn new(data: Vec<Product>, reporter: Arc<Mutex<Reporter>>) -> Self {
        Self { data, reporter }
    }

    fn data(&self) -> Vec<Product> {
        self.data.clone()
    }
}

impl contracts::ProductRepo for TestProductRepo {
    fn find_one(&self, id: i32) -> Result<Product, Error> {
        match self.data().into_iter().find(|item| item.id == id) {
            None => {
                Err(Error::NotFound)
            },

            Some(product) => Ok(product)
        }
    }

    fn list(&self) -> Result<Vec<ListedProduct>, Error> {
        self.reporter.lock()
            .expect("Error locking reporter")
            .register_fn_call("product_repo.list");
        
        Ok(self.data().iter()
            .map(|p| ListedProduct::from(p.clone()))
            .collect()
        )
    }

    fn list_paginated(&self, page: i64, per_page: i64) -> Result<(i64, Vec<ListedProduct>), Error> {
        let products = self.data();
        let entries = products.len() as i64;

        let grouped: Vec<ListedProduct> = get_paginated_page(&products, page, per_page)
            .iter()
            .map(|p| ListedProduct::from(p.clone()))
            .collect();

        self.reporter.lock()
            .expect("Error locking reporter")
            .register_fn_call("product_repo.list_paginated");

        Ok((entries, grouped))
    }

    fn insert(&self, new_product: NewProduct) -> Result<Product, Error> {
        let products = self.data();
        let NewProduct { name, description, url, price, available } = new_product;
        let id = match products.last() {
            None => 1,
            Some(p) => p.id + 1,
        };

        let product = Product { id, name, description, url, price, available };

        Ok(product)
    }

    fn delete(&self, id: i32) -> Result<usize, Error> {
        let products = self.data();

        match products.iter().find(|p| p.id == id) {
            None => Err(Error::NotFound),
            Some(_) => Ok(1),
        }
    }

    fn update(&self, product: &Product) -> Result<Product, Error> {
        Ok(product.clone())
    }

    fn insert_many(&self, products: Vec<NewProduct>) -> Result<usize, Error> {
        self.reporter.lock()
            .expect("Locking Reporter failed")
            .register_fn_call("product_repo.insert_many");

        Ok(products.len())
    }
}

