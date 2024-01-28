use salvo::prelude::*;
use salvo::http::form::FormData;
use crate::api::errors as api_errors;
use super::{repo, create_token};
use crate::api::responses;

#[handler]
pub async fn authenticate(req: &mut Request, _depot: &mut Depot, res: &mut Response) {
    match req.form_data().await {
        Err(error) => api_errors::render_form_data_error(res, error),

        Ok(form_data) => match cast_login_form_data(form_data) {
            Err(error) => api_errors::render_cast_error(res, error),

            Ok((email_candidate, password_candidate)) => match repo::validate(email_candidate, password_candidate) {
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

fn cast_login_form_data(form_data: &FormData) -> Result<(&str, &str), api_errors::Error> {
    use api_errors::Error::FieldNotFound;

    let casted_email = form_data.fields.get("email")
        .ok_or(FieldNotFound("email"))?;

    let casted_password = form_data.fields.get("password")
        .ok_or(FieldNotFound("password"))?;

    Ok((casted_email, casted_password))
}

