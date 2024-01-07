use salvo::prelude::*;
use salvo::http::form::FormData;
use crate::api::auth::JwtClaims;
use crate::api::{errors as api_errors, responses as api_responses, utils};
use crate::models::Updatable;
use super::models::NewWishlist;
use super::repo;

#[handler]
pub fn list_wishlist(_req: &mut Request, depot: &mut Depot, res: &mut Response) {
    match utils::get_user_id(depot) {
        None => api_errors::render_get_user_id_not_found(res),

        Some(user_id) => match repo::list_wishlists(user_id) {
            Err(error) => api_errors::render_db_retrieving_error(res, error, "wishlists"),

            Ok(wishlists) => api_responses::render_collection(res, wishlists),
        },
    }
}

#[handler]
pub fn show_wishlist(req: &mut Request, depot: &mut Depot, res: &mut Response) {
    match (utils::get_req_param(req, "id"), utils::get_user_id(depot)) {
        (Err(error), _) => api_errors::render_form_data_error(res, error),

        (_, None) => api_errors::render_get_user_id_not_found(res),

        (Ok(id), Some(user_id)) => match req.query::<String>("detailed") {
            None => match repo::find_wishlist(id, user_id) {
                Err(_) => api_errors::render_resource_not_found(res, "wishlist"),

                Ok(wishlist) => api_responses::render_resource(res, wishlist),
            },
            Some(_) => match repo::find_detailed_wishlist(id, user_id) {
                Err(_) => api_errors::render_resource_not_found(res, "wishlist"),

                Ok(wishlist) => api_responses::render_resource(res, wishlist),
            },
        },
    }
}

#[handler]
pub async fn create_wishlist(req: &mut Request, depot: &mut Depot, res: &mut Response) {
    match req.form_data().await {
        Err(error) => api_errors::render_form_data_error(res, error),

        Ok(form_data) => {
            let data = depot.jwt_auth_data::<JwtClaims>().unwrap();
            let user_id = data.claims.id;

            match cast_form_data_to_new_wishlist(form_data, user_id) {
                Err(error) => api_errors::render_cast_error(res, error),

                Ok(new_wishlist) => match repo::insert_wishist(new_wishlist) {
                    Err(error) => api_errors::render_db_insert_error(res, error, "wishlist"),

                    Ok(wishlist) => api_responses::render_resource_created(res, wishlist),
                },
            }
        }
    }
}

#[handler]
pub async fn update_wishlist(req: &mut Request, depot: &mut Depot, res: &mut Response) {
    match (utils::get_req_param(req, "id"), utils::get_user_id(depot)) {
        (Err(error), _) => api_errors::render_form_data_error(res, error),

        (_, None) => api_errors::render_get_user_id_not_found(res),

        (Ok(id), Some(user_id)) => match req.form_data().await {
            Err(error) => api_errors::render_form_data_error(res, error),

            Ok(form_data) => match repo::find_wishlist(id, user_id) {
                Err(_) => api_errors::render_resource_not_found(res, "wishlist"),

                Ok(wishlist) => {
                    let updated_wishlist = wishlist.merge(form_data);

                    match repo::update_wishist(&updated_wishlist, user_id) {
                        Err(error) => api_errors::render_db_update_error(res, error, "wishlist"),

                        Ok(updated_wishlist) => {
                            api_responses::render_resource_updated(res, updated_wishlist)
                        }
                    }
                }
            },
        },
    }
}

#[handler]
pub fn delete_wishlist(req: &mut Request, depot: &mut Depot, res: &mut Response) {
    match (utils::get_req_param(req, "id"), utils::get_user_id(depot)) {
        (Err(error), _) => api_errors::render_parse_field_error(res, error, "id"),

        (_, None) => api_errors::render_get_user_id_not_found(res),

        (Ok(id), Some(user_id)) => match repo::delete_wishlist(id, user_id) {
            Err(error) => api_errors::render_db_delete_error(res, error, "wishlist"),

            Ok(total_deleted) => match total_deleted {
                0 => api_errors::render_resource_not_found(res, "wishlist"),

                _other => api_responses::render_resource_deleted(res, total_deleted),
            },
        },
    }
}

fn cast_form_data_to_new_wishlist(
    form_data: &FormData,
    user_id: i32,
) -> Result<NewWishlist, api_errors::Error> {
    use api_errors::Error::FieldNotFound;

    let title = form_data
        .fields
        .get("title")
        .ok_or(FieldNotFound("title"))?;

    let description = form_data
        .fields
        .get("description")
        .ok_or(FieldNotFound("description"))?;

    let new_wishlist = NewWishlist {
        title,
        description: Some(description),
        user_id,
    };

    Ok(new_wishlist)
}
