use salvo::prelude::*;
use salvo::http::form::FormData;
use crate::api::auth::JwtClaims;
use crate::api::errors::ApiResult;
use crate::api::utils::pagination::Pagination;
use crate::api::validations::{FormValidator, Validator};
use crate::api::{errors as api_errors, responses as api_responses, utils};
use crate::models::Updatable;
use super::models::NewWishlist;
use super::repo;

#[handler]
pub fn list_wishlists(req: &mut Request, _depot: &mut Depot, res: &mut Response) {
    match req.query::<i64>("per_page") {
        None => match repo::list_wishlists() {
            Err(_) => api_errors::render_resource_not_found(res, "wishlist"),

            Ok(wishlists) => api_responses::render_collection(res, wishlists),
        },
        Some(per_page) => {
            let page = req.query::<i64>("page").unwrap_or(1);

            match repo::list_wishlists_paginate(page, per_page) {
                Err(error) => api_errors::render_db_retrieving_error(res, error, "wishlists"),

                Ok((entries, wishlists)) =>
                    api_responses::render_collection_paginated(res, wishlists, Pagination::new(page, per_page, entries))
            }
        }
    }
}

#[handler]
pub fn list_user_wishlists(_req: &mut Request, depot: &mut Depot, res: &mut Response) {
    match utils::get_user_id(depot) {
        None => api_errors::render_get_user_id_not_found(res),

        Some(user_id) => match repo::list_user_wishlists(user_id) {
            Err(error) => api_errors::render_db_retrieving_error(res, error, "wishlists"),

            Ok(wishlists) => api_responses::render_collection(res, wishlists),
        },
    }
}

#[handler]
pub fn show_wishlist(req: &mut Request, depot: &mut Depot, res: &mut Response) {
    match (req.param::<i32>("id"), utils::get_user_id(depot)) {
        (None, _) => api_errors::render_resource_not_found(res, "id"),

        (_, None) => api_errors::render_get_user_id_not_found(res),

        (Some(id), Some(user_id)) => match req.query::<String>("detailed") {
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
    match (req.param::<i32>("id"), utils::get_user_id(depot)) {
        (None, _) => api_errors::render_resource_not_found(res, "id"),

        (_, None) => api_errors::render_get_user_id_not_found(res),

        (Some(id), Some(user_id)) => match req.form_data().await {
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
    match (req.param::<i32>("id"), utils::get_user_id(depot)) {
        (None, _) => api_errors::render_resource_not_found(res, "id"),

        (_, None) => api_errors::render_get_user_id_not_found(res),

        (Some(id), Some(user_id)) => match repo::delete_wishlist(id, user_id) {
            Err(error) => api_errors::render_db_delete_error(res, error, "wishlist"),

            Ok(total_deleted) => match total_deleted {
                0 => api_errors::render_resource_not_found(res, "wishlist"),

                _other => api_responses::render_db_execution(res, total_deleted),
            },
        },
    }
}

fn cast_form_data_to_new_wishlist(form_data: &FormData, user_id: i32) -> ApiResult<NewWishlist> {
    let validator = FormValidator(form_data);

    let new_wishlist = NewWishlist {
        title: validator.string("title")?,
        description: validator.optional_string("description")?,
        user_id,
    };

    Ok(new_wishlist)
}
