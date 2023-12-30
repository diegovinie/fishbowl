
use salvo::prelude::*;
use salvo::http::form::FormData;
use crate::api::{utils, errors as api_errors};
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

                Ok(wishes) => {
                    res.render(Json(wishes));
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

                        Ok(wish) => {
                            res.status_code(StatusCode::ACCEPTED);
                            res.render(Json(wish));
                        }
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
