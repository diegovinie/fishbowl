use salvo::prelude::*;
use salvo::http::form::FormData;
use crate::api::utils;
use crate::models::Updatable;
use super::models::NewProduct;
use super::repo;

#[derive(Debug)]
enum Error<'a> {
    FieldNotFound(&'a str),
    ParseFloatErr(&'a str),
}
    
#[handler]
pub fn list_products(_req: &mut Request, _depot: &mut Depot, res: &mut Response) {
    let products = repo::list_products()
        .expect("Error loading products");

    res.render(Json(products))

}

#[handler]
pub async fn add_product(req: &mut Request, _depot: &mut Depot, res: &mut Response) {
    match req.form_data().await {
        Err(error) => {
            res.status_code(StatusCode::BAD_REQUEST);
            res.render(format!("Error getting the form data: {error}"));
        },
        Ok(form_data) => match cast_form_data_to_new_product(form_data) {
            Err(error) => {
                res.status_code(StatusCode::BAD_REQUEST);
                res.render(format!("Error parsing the form data fields: {error:?}"));
            },
            Ok(new_product) => match repo::insert_product(new_product) {
                Err(error) => {
                    res.status_code(StatusCode::INTERNAL_SERVER_ERROR);
                    res.render(format!("Error inserting product: {error}"));
                },
                Ok(product) => {
                    res.render(Json(product));
                }
            }
        }
    }
}

#[handler]
pub fn show_product(req: &mut Request, _depot: &mut Depot, res: &mut Response) {
    match utils::get_req_param::<i32>(req, "id") {
        Err(error) => {
            res.status_code(StatusCode::BAD_REQUEST);
            res.render(format!("Incorrect <id>: {error}"));
        },
        Ok(id) => match repo::find_product(id) {
            Err(_) => {
                res.status_code(StatusCode::NOT_FOUND);
            },
            Ok(product) => {
                res.render(Json(product));
            }
        }
    }
}

#[handler]
pub fn remove_product(req: &mut Request, _depot: &mut Depot, res: &mut Response) {
    match utils::get_req_param(req, "id") {
        Err(error) => {
            res.status_code(StatusCode::BAD_REQUEST);
            res.render(format!("Incorrect <id>: {error}"));
        },
        Ok(id) => match repo::delete_product(id) {
            Err(error) => {
                res.status_code(StatusCode::INTERNAL_SERVER_ERROR);
                res.render(format!("Error deleting `product`: {error}"));
            },
            Ok(total_deleted) => match total_deleted {
                0 => {
                    res.status_code(StatusCode::NOT_FOUND);
                    res.render(format!("Nothing was deleted"));
                },
                1 => {
                    res.status_code(StatusCode::ACCEPTED);
                },
                _other => {
                    res.render(format!("Total deleted: {}", total_deleted));
                }
            }
        }
    }
}

#[handler]
pub async fn update_product(req: &mut Request, _depot: &mut Depot, res: &mut Response) {
    match utils::get_req_param(req, "id") {
        Err(error) => {
            res.status_code(StatusCode::BAD_REQUEST);
            res.render(format!("Incorrect <id>: {error}"));
        },
        Ok(id) => match req.form_data().await {
            Err(error) => {
                res.status_code(StatusCode::BAD_REQUEST);
                res.render(format!("Error getting the form data: {error}"));
            },
            Ok(form_data) => match repo::find_product(id) {
                Err(_) => {
                    res.status_code(StatusCode::NOT_FOUND);
                    res.render("Error `product` not found");
                },
                Ok(product) => {
                    let product_updated = product.merge(form_data);

                    match repo::update_product(&product_updated) {
                        Err(error) => {
                            res.status_code(StatusCode::INTERNAL_SERVER_ERROR);
                            res.render(format!("Error updating `product`: {error}"));
                        },
                        Ok(product_updated) => {
                            res.status_code(StatusCode::ACCEPTED);
                            res.render(Json(product_updated));
                        }
                    }
                }                
            }
        }   
    }
}

fn cast_form_data_to_new_product(form_data: &FormData) -> Result<NewProduct, Error> {
    let name = form_data.fields.get("name")
        .ok_or(Error::FieldNotFound("name"))?;

    let description = form_data.fields.get("description")
        .ok_or(Error::FieldNotFound("description"))?;

    let price: f32 = form_data.fields.get("price")
        .ok_or(Error::FieldNotFound("price"))?
        .parse()
        .map_err(|_| Error::ParseFloatErr("price"))?;

    let new_product = NewProduct { name, description, price, available: false };

    Ok(new_product)
}
