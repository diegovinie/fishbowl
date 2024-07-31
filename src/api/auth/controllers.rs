use salvo::prelude::*;
use salvo::http::form::FormData;
use time::{OffsetDateTime, Duration};
use crate::api::errors::{ApiError, ApiResult};
use crate::api::responses as api_responses;
use crate::api::resources::users::models::NewUser;
use crate::api::utils::{get_db, get_notifier};
use crate::api::validations::{FormValidator, Validator};
use super::models::{ActivateUserAction, ActivateUserClaims};
use super::{create_bearer_token, decode_token, encode_token};
use crate::api::responses;

#[handler]
pub async fn authenticate(req: &mut Request, depot: &mut Depot, res: &mut Response) -> ApiResult<()> {
    let repo = get_db(depot)?.auth_repo();

    let form_data = req.form_data().await?;

    let (email_candidate, password_candidate) = cast_login_form_data(form_data)?;

    let user = repo.validate(&email_candidate, &password_candidate).ok_or(ApiError::InvalidCredentials)?;

    let token = create_bearer_token(&user)?;

    responses::render_authentication(res, user.into(), token);

    Ok(())
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

    let new_user = cast_registry_data(form_data)?;

    let user = repo.insert(new_user)?;

    let notifier = get_notifier(depot)?;

    let claims = ActivateUserClaims {
        id: user.id,
        email: user.email.clone(),
        action: ActivateUserAction::Activate,
        exp: (OffsetDateTime::now_utc() + Duration::minutes(1)).unix_timestamp(),
    };

    let token = encode_token(claims).unwrap();

    notifier.send(&user, format!("token={token}"));

    api_responses::render_resource_created(res, user);

    Ok(())
}

#[handler]
pub fn activate(req: &Request, depot: &Depot, res: &mut Response) -> ApiResult<()> {
    let repo = get_db(depot)?.auth_repo();

    match req.query("token") {
        None => {
            res.status_code(StatusCode::NO_CONTENT);
            res.render("No changes");

            Ok(())
        },

        Some(token) => {
            let data = decode_token::<ActivateUserClaims>(token)?;

            match data.claims.action {
                ActivateUserAction::Activate => {
                    let total = repo.activate(data.claims.id, &data.claims.email).unwrap();

                    api_responses::render_db_execution(res, total);
                    
                    Ok(())
                }
            }
        }
    }
}

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
