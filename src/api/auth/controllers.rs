use salvo::prelude::*;
use salvo::http::form::FormData;
use crate::api::errors::{self as api_errors, ApiResult};
use crate::api::responses as api_responses;
use crate::api::resources::users::models::NewUser;
use crate::api::utils::{get_db, get_notifier};
use crate::api::validations::{FormValidator, Validator};
use super::{repo, create_token};
use crate::api::responses;

#[handler]
pub async fn authenticate(req: &mut Request, _depot: &mut Depot, res: &mut Response) {
    match req.form_data().await {
        Err(error) => api_errors::render_form_data_error(res, error),

        Ok(form_data) => match cast_login_form_data(form_data) {
            Err(error) => api_errors::render_cast_error(res, error),

            Ok((email_candidate, password_candidate)) => match repo::validate(&email_candidate, &password_candidate) {
                None => api_errors::render_auth_validation_none(res),

                Some(user) => match create_token(user.clone()) {
                    Err(error) => api_errors::render_auth_create_token_error(res, error),

                    Ok(token) => responses::render_authentication(res, user.into(), token),
                }
            },
        },
    }
}

#[handler]
pub fn handle_auth(_req: &mut Request, depot: &mut Depot, res: &mut Response) {
    match depot.jwt_auth_state() {
        JwtAuthState::Authorized => {},
        JwtAuthState::Unauthorized => {
            res.status_code(StatusCode::UNAUTHORIZED);
        },
        JwtAuthState::Forbidden => {
            res.status_code(StatusCode::FORBIDDEN);
        },
    }
}

#[handler]
pub async fn signup(req: &mut Request, depot: &Depot, res: &mut Response) -> ApiResult<()> {
    let repo = get_db(depot)?.user_repo();

    let form_data = req.form_data().await?;

    let new_user = cast_registry_data(&form_data)?;

    let user = repo.insert(new_user)?;

    let notifier = get_notifier(depot)?;

    notifier.send(&user, "finally!".to_string());

    api_responses::render_resource_created(res, user);

    Ok(())
}

// c: dispatch message

// verify_user
// 

fn cast_login_form_data(form_data: &FormData) -> ApiResult<(String, String)> {
    let validator = FormValidator(form_data);

    let email = validator.string("email")?;
    let password = validator.string("password")?;

    Ok((email, password))
}

fn cast_registry_data(form_data: &FormData) -> ApiResult<NewUser> {
    let validator = FormValidator(form_data);

    let name = validator.string("name")?;
    let email = validator.string("email")?;
    let password = validator.password("password")?;

    Ok(NewUser { name, email, password, active: false })
}
