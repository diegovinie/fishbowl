mod models;
mod repo;

use jsonwebtoken::EncodingKey;
use salvo::http::form::FormData;
use salvo::prelude::*;
use salvo::jwt_auth::{ConstDecoder, QueryFinder, HeaderFinder};
use serde::{Deserialize, Serialize};
use time::{OffsetDateTime, Duration};
use crate::api::errors as api_errors;
use crate::models::Role;
use self::models::User;

const SECRET_KEY: &str = "YOUR SECRET_KEY";

#[derive(Debug, Serialize, Deserialize)]
pub struct JwtClaims {
    pub username: String,
    pub role: Role,
    pub id: i32,
    exp: i64,
}

pub fn decode_token() -> JwtAuth<JwtClaims, ConstDecoder> {
    JwtAuth::new(ConstDecoder::from_secret(SECRET_KEY.as_bytes()))
        .finders(vec![
            Box::new(HeaderFinder::new()),
            Box::new(QueryFinder::new("jwt_token")),
        ])
        .force_passed(true)
}

#[handler]
pub async fn authenticate(req: &mut Request, _depot: &mut Depot, res: &mut Response) {
    match req.form_data().await {
        Err(error) => api_errors::render_form_data_error(res, error),

        Ok(form_data) => match cast_login_form_data(form_data) {
            Err(error) => api_errors::render_cast_error(res, error),

            Ok((email_candidate, password_candidate)) => match repo::validate(email_candidate, password_candidate) {
                None => api_errors::render_auth_validation_none(res),

                Some(user) => match create_token(user) {
                    Err(error) => api_errors::render_auth_create_token_error(res, error),

                    Ok(token) => {
                        res.render(token);
                    }
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

fn create_token(user: User) -> Result<String, jsonwebtoken::errors::Error> {
    let User { name: username, id, role, .. } = user;
    let exp = OffsetDateTime::now_utc() + Duration::hours(1);

    let claims = JwtClaims {
        username,
        id,
        role: Role::from(role.as_str()),
        exp: exp.unix_timestamp(),
    };

    let header = jsonwebtoken::Header::default();
    let key = EncodingKey::from_secret(SECRET_KEY.as_bytes());

    jsonwebtoken::encode(&header, &claims, &key)
}
