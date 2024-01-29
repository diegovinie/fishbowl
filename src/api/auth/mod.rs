pub mod models;
pub mod controllers;
mod repo;

use jsonwebtoken::EncodingKey;
use salvo::prelude::*;
use salvo::jwt_auth::{ConstDecoder, QueryFinder, HeaderFinder};
use serde::{Deserialize, Serialize};
use time::{OffsetDateTime, Duration};
use crate::models::Role;
use self::controllers::{authenticate, signup};
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

pub fn create_token(user: User) -> Result<String, jsonwebtoken::errors::Error> {
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

pub fn get_router() -> Router {
    Router::with_path("api/v1/auth")
        .post(authenticate)
        .push(Router::with_path("signup").post(signup))
}
