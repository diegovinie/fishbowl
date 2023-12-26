use salvo::prelude::*;
use salvo::http::form::FormData;
use crate::api::auth::JwtClaims;
use crate::api::{utils, Error};
use crate::models::Updatable;
use super::models::NewWishlist;
use super::repo;

#[handler]
pub fn list_wishlist(_req: &mut Request, depot: &mut Depot, res: &mut Response) {
    match utils::get_user_id(depot) {
        None => {
            res.status_code(StatusCode::INTERNAL_SERVER_ERROR);
            res.render(format!("Couldn't read `user_id` from depot"));
        },
        Some(user_id) => match repo::list_wishlists(user_id) {
            Err(error) => {
                res.status_code(StatusCode::INTERNAL_SERVER_ERROR);
                res.render(format!("Error loading wishlists: {error}"));
            },
            Ok(wishlists) => {
                res.render(Json(wishlists));
            }
        }
    }
}

#[handler]
pub fn show_wishlist(req: &mut Request, depot: &mut Depot, res: &mut Response) {
    match (utils::get_req_param(req, "id"), utils::get_user_id(depot)) {
        (Err(error), _) => {
            res.status_code(StatusCode::BAD_REQUEST);
            res.render(format!("Error getting the form data: {error}"));
        },
        (_, None) => {
            res.status_code(StatusCode::INTERNAL_SERVER_ERROR);
            res.render(format!("Couldn't read `user_id` from depot"));

        },
        (Ok(id), Some(user_id)) => match repo::find_wishlist(id, user_id) {
            Err(_) => {
                res.status_code(StatusCode::NOT_FOUND);
            },
            Ok(wishlist) => {
                res.render(Json(wishlist));
            }
        }
    }
}

#[handler]
pub async fn create_wishlist(req: &mut Request, depot: &mut Depot, res: &mut Response) {
    match req.form_data().await {
        Err(error) => {
            res.status_code(StatusCode::BAD_REQUEST);
            res.render(format!("Error getting the form data: {error}"));
        },
        Ok(form_data) => {
            let data = depot.jwt_auth_data::<JwtClaims>().unwrap();
            let user_id = data.claims.id;

            match cast_form_data_to_new_wishlist(form_data, user_id) {
                Err(error) => {
                    res.status_code(StatusCode::BAD_REQUEST);
                    res.render(format!("Error parsing the form data fields: {error:?}"));
                },
                Ok(new_wishlist) => match repo::insert_wishist(new_wishlist) {
                    Err(error) => {
                        res.status_code(StatusCode::INTERNAL_SERVER_ERROR);
                        res.render(format!("Error inserting `wishlist`: {error}"));
                    },
                    Ok(wishlist) => {
                        res.render(Json(wishlist));
                    }
                }   
            }
        }
    }
}

#[handler]
pub async fn update_wishist(req: &mut Request, depot: &mut Depot, res: &mut Response) {
    match (utils::get_req_param(req, "id"), utils::get_user_id(depot)) {
        (Err(error), _) => {
            res.status_code(StatusCode::BAD_REQUEST);
            res.render(format!("Error getting the form data: {error}"));
        },
        (_, None) => {
            res.status_code(StatusCode::INTERNAL_SERVER_ERROR);
            res.render(format!("Couldn't read `user_id` from depot"));

        },    
        (Ok(id), Some(user_id)) => match req.form_data().await {
            Err(error) => {
                res.status_code(StatusCode::BAD_REQUEST);
                res.render(format!("Error getting the form data: {error}"));
            },
            Ok(form_data) => match repo::find_wishlist(id, user_id) {
                Err(_) => {
                    res.status_code(StatusCode::NOT_FOUND);
                    res.render("Error `wishlist` not found");
                },
                Ok(wishlist) => {
                    let updated_wishlist = wishlist.merge(form_data);

                    match repo::update_wishist(&updated_wishlist, user_id) {
                        Err(error) => {
                            res.status_code(StatusCode::INTERNAL_SERVER_ERROR);
                            res.render(format!("Error updating `wishlist`: {error}"));
                        },
                        Ok(updated_wishlist) => {
                            res.status_code(StatusCode::ACCEPTED);
                            res.render(Json(updated_wishlist));
                        }
                    }
                }
            }
        }
    }
}

#[handler]
pub fn delete_wishlist(req: &mut Request, depot: &mut Depot, res: &mut Response) {
    match (utils::get_req_param(req, "id"), utils::get_user_id(depot)) {
        (Err(error), _) => {
            res.status_code(StatusCode::BAD_REQUEST);
            res.render(format!("Incorrect <id>: {error}"));
        },
        (_, None) => {
            res.status_code(StatusCode::INTERNAL_SERVER_ERROR);
            res.render(format!("Couldn't read `user_id` from depot"));
        },
        (Ok(id), Some(user_id)) => match repo::delete_wishlist(id, user_id) {
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

fn cast_form_data_to_new_wishlist(form_data: &FormData, user_id: i32) -> Result<NewWishlist, Error> {
    let title = form_data.fields.get("title")
        .ok_or(Error::FieldNotFound("title"))?;

    let description = form_data.fields.get("description")
        .ok_or(Error::FieldNotFound("description"))?;

    let new_wishlist = NewWishlist { title, description: Some(description), user_id };

    Ok(new_wishlist)
}
