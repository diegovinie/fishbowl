
use salvo::prelude::*;
use salvo::http::form::FormData;
use crate::api::errors::{ApiError, ApiResult};
use crate::api::utils::{get_db, get_user_id};
use crate::api::validations::{FormValidator, Validator};
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
pub fn show_wish(req: &Request, depot: &Depot, res: &mut Response) -> ApiResult<()> {
    let db = get_db(depot)?;

    let detailed = req.query::<bool>("detailed").unwrap_or_default();

    if detailed {
        let user_id = utils::get_user_id(depot).ok_or(ApiError::NotAllowed("User not found".to_string()))?;

        let wishlist_id = req.param::<i32>("wishlist_id").ok_or(ApiError::FieldNotFound("wishlist_id".to_string()))?;

        let wishlist = db.wishlist_repo().find_one(wishlist_id)?;

        if wishlist.user_id != user_id {
            return Err(ApiError::NotAllowed("User is not allowed".to_string()));
        }
    }

    let id = req.param::<i32>("id").ok_or(ApiError::FieldNotFound("id".to_string()))?;

    let wish = db.wish_repo().find_one_expanded(id)?;

    api_responses::render_resource(res, wish);

    Ok(())
}

#[handler]
pub async fn create_wish(req: &mut Request, depot: &Depot, res: &mut Response) -> ApiResult<()> {
    let db = get_db(depot)?;

    let form_data = req.form_data().await?;

    let user_id = get_user_id(depot).unwrap_or_default();

    let new_wish = cast_form_data_to_new_wish(form_data)?;

    let wishlist = db.wishlist_repo().find_one(new_wish.wishlist_id)?;

    if wishlist.user_id != user_id {
        return Err(ApiError::Deserializer("User doesn't own the wishlist".to_string()));
    }

    let wish = db.wish_repo().insert(new_wish)?;

    api_responses::render_resource_created(res, wish);

    Ok(())
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

                        _other => api_responses::render_db_execution(res, total_deleted),
                    }
                }
            }
        }
    }
}

fn cast_form_data_to_new_wish(form_data: &FormData) -> ApiResult<NewWish> {
    let validator = FormValidator(form_data);

    let new_wish = NewWish { 
        wishlist_id: validator.integer("wishlist_id")?,
        product_id: validator.integer("product_id")?,
    };

    Ok(new_wish)
}
