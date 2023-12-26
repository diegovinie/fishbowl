mod models;
mod repo;

use jsonwebtoken::EncodingKey;
use salvo::http::form::FormData;
use salvo::prelude::*;
use salvo::jwt_auth::{ConstDecoder, QueryFinder, HeaderFinder};
use serde::{Deserialize, Serialize};
use time::{OffsetDateTime, Duration};
use super::Error;

const SECRET_KEY: &str = "YOUR SECRET_KEY";

#[derive(Debug, Serialize, Deserialize)]
pub struct JwtClaims {
    pub username: String,
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
        Err(error) => {
            res.status_code(StatusCode::BAD_REQUEST);
            res.render(format!("Error getting the form data: {error}"));
        },
        Ok(form_data) => match cast_login_form_data(form_data) {
            Err(error) => {
                res.status_code(StatusCode::BAD_REQUEST);
                res.render(format!("Error parsing the form data fields: {error:?}"));    
            },
            Ok((email_candidate, password_candidate)) => match repo::validate(email_candidate, password_candidate) {
                None => {
                    res.status_code(StatusCode::NOT_ACCEPTABLE);
                    res.render("Authentication failed");
                },
                Some(user) => match create_token(user.email, user.id) {
                    Err(error) => {
                        res.status_code(StatusCode::INTERNAL_SERVER_ERROR);
                        res.render(format!("Error creating token: {error:?}")); 
                    },
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

fn cast_login_form_data(form_data: &FormData) -> Result<(&str, &str), Error> {
    let casted_email = form_data.fields.get("email")
        .ok_or(Error::FieldNotFound("email"))?;

    let casted_password = form_data.fields.get("password")
        .ok_or(Error::FieldNotFound("password"))?;   

    Ok((casted_email, casted_password))

}

fn create_token(username: String, id: i32) -> Result<String, jsonwebtoken::errors::Error> {
    let exp = OffsetDateTime::now_utc() + Duration::hours(1);

    let claims = JwtClaims {
        username,
        id,
        exp: exp.unix_timestamp(),
    };

    let header = jsonwebtoken::Header::default();
    let key = EncodingKey::from_secret(SECRET_KEY.as_bytes());

    jsonwebtoken::encode(&header, &claims, &key)
}
