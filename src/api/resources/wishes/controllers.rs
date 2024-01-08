
use salvo::prelude::*;
use salvo::http::form::FormData;
use crate::api::{utils, errors as api_errors, responses as api_responses};
use super::models::NewWish;
use super::repo;
use crate::api::resources::wishlists::repo::find_wishlist;

#[handler]
pub fn list_wishes(req: &mut Request, depot: &mut Depot, res: &mut Response) {
    match (utils::get_user_id(depot), req.param::<i32>("wishlist_id")) {
        (None, _) => api_errors::render_get_user_id_not_found(res),

        (_, None) => api_errors::render_resource_not_found(res, "associated wishlist"),

        (Some(user_id), Some(wishlist_id)) => match find_wishlist(wishlist_id, user_id) {
            Err(_) => api_errors::render_db_resource_not_associated(res, "wishlist"),

            Ok(_) => match repo::list_wishes_from_wishlist(wishlist_id) {
                Err(error) => api_errors::render_db_retrieving_error(res, error, "wishes"),

                Ok(wishes) => api_responses::render_collection(res, wishes)
            }
        }
    }
}

#[handler]
pub fn show_wish(req: &mut Request, depot: &mut Depot, res: &mut Response) {
    match (utils::get_user_id(depot), req.param::<i32>("wishlist_id")) {
        (None, _) => api_errors::render_get_user_id_not_found(res),

        (_, None) => api_errors::render_resource_not_found(res, "associated wishlist"),

        (Some(user_id), Some(wishlist_id)) => match find_wishlist(wishlist_id, user_id) {
            Err(_) => api_errors::render_db_resource_not_associated(res, "wishlist"),

            Ok(_wishlist) => match req.param::<i32>("id") {
                None => api_errors::render_resource_not_found(res, "wish_id"),

                Some(id) => match repo::find_wish(id) {
                    Err(error) => api_errors::render_db_retrieving_error(res, error, "wish"),

                    Ok(wish) => api_responses::render_resource(res, wish)
                }
            }
        }
    }
}

#[handler]
pub async fn create_wish(req: &mut Request, depot: &mut Depot, res: &mut Response) {
    match (req.param::<i32>("wishlist_id"), utils::get_user_id(depot)) {
        (_, None) => api_errors::render_get_user_id_not_found(res),

        (None, _) => api_errors::render_resource_not_found(res, "associated wishlist"),

        (Some(w_id), Some(user_id)) => match req.form_data().await {
            Err(error) => api_errors::render_form_data_error(res, error),

            Ok(form_data) => match cast_form_data_to_new_wish(form_data) {
                Err(error) => api_errors::render_cast_error(res, error),

                Ok(new_wish) => match (find_wishlist(w_id, user_id), new_wish.wishlist_id == w_id) {
                    (Err(_), _) => api_errors::render_db_resource_not_associated(res, "wishlist"),

                    (_, false) => api_errors::render_inconsistency_error(res, "wishlist_id"),

                    (Ok(_), true) => match repo::insert_wish(new_wish) {
                        Err(error) => api_errors::render_db_insert_error(res, error, "wish"),

                        Ok(wish) => api_responses::render_resource_created(res, wish)
                    }
                }
            }
        }
    }
}

#[handler]
pub fn delete_wish(req: &mut Request, depot: &mut Depot, res: &mut Response) {
    match (req.param::<i32>("wishlist_id"), utils::get_user_id(depot)) {
        (None, _) => api_errors::render_resource_not_found(res, "associated wishlist"),

        (_, None) => api_errors::render_get_user_id_not_found(res),

        (Some(wishlist_id), Some(user_id)) => match find_wishlist(wishlist_id, user_id) {
            Err(_) => api_errors::render_db_resource_not_associated(res, "wishlist"),

            Ok(_) => match req.param::<i32>("id") {
                None => api_errors::render_resource_not_found(res, "wish"),

                Some(id) => match repo::delete_wish(id) {
                    Err(error) => api_errors::render_db_delete_error(res, error, "wish"),

                    Ok(total_deleted) => match total_deleted {
                        0 => api_errors::render_resource_not_found(res, "wish"),

                        _other => api_responses::render_resource_deleted(res, total_deleted),
                    }
                }
            }
        }
    }
}

fn cast_form_data_to_new_wish(form_data: &FormData) -> Result<NewWish, api_errors::Error> {
    use api_errors::Error::{FieldNotFound, ParseIntErr};

    let wishlist_id: i32 = form_data.fields.get("wishlist_id")
        .ok_or(FieldNotFound("wishlist_id"))?
        .parse()
        .map_err(|_| ParseIntErr("wishlist_id"))?;

    let product_id: i32 = form_data.fields.get("product_id")
        .ok_or(FieldNotFound("product_id"))?
        .parse()
        .map_err(|_| ParseIntErr("product_id"))?;

    let new_wish = NewWish { wishlist_id, product_id };

    Ok(new_wish)
}
