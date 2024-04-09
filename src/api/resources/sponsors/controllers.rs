use std::sync::Arc;

use salvo::{prelude::*, http::form::FormData};
use crate::api::validations::{Validator, FormValidator};
use crate::api::{responses as api_responses, utils};
use crate::api::errors::{ApiResult, ApiError};
use crate::services::database::contracts::DatabaseService;
use super::models::NewSponsor;

#[handler]
pub async fn list_sponsors_wish(req: &mut Request, depot: &mut Depot, res: &mut Response) -> ApiResult<()> {
    let repo = get_db(depot)?.sponsor_repo();

    let wish_id = req.param("wish_id").ok_or(ApiError::FieldNotFound("wish_id".to_string()))?;

    let sponsors = repo.list_by_wish(wish_id)?;

    api_responses::render_collection(res, sponsors);
    
    Ok(())
}

#[handler]
pub async fn add_sponsor(req: &mut Request, depot: &Depot, res: &mut Response) -> ApiResult<()> {
    let repo = get_db(depot)?.sponsor_repo();

    let form_data = req.form_data().await?;

    let user_id = utils::get_user_id(depot).unwrap_or_default();

    let new_product = cast_form_data_to_new_sponsor(form_data, user_id)?;

    let product = repo.insert(new_product)?;

    api_responses::render_resource(res, product);

    Ok(())
}

fn cast_form_data_to_new_sponsor(form_data: &FormData, user_id: i32) -> ApiResult<NewSponsor> {
    let validator = FormValidator(form_data);

    let new_sponsor = NewSponsor {
        user_id,
        wish_id: validator.integer("wish_id")?,
        amount: validator.float("amount")?,
        leader: validator.optional_boolean("leader")?,
    };

    Ok(new_sponsor)
}

fn get_db(depot: &Depot) -> ApiResult<&Arc<dyn DatabaseService>> {
    use crate::api::errors::InjectionError;

    let service = depot.obtain::<Arc<dyn DatabaseService>>()
        .map_err(|_| ApiError::Injection(InjectionError))?;

    Ok(service)
}