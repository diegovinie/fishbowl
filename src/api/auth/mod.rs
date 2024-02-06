pub mod models;
pub mod controllers;
mod repo;

use jsonwebtoken::{Algorithm, DecodingKey, EncodingKey, Validation};
use salvo::prelude::*;
use salvo::jwt_auth::{ConstDecoder, QueryFinder, HeaderFinder};
use serde::de::DeserializeOwned;
use serde::{Deserialize, Serialize};
use time::{OffsetDateTime, Duration};
use crate::models::Role;
use self::controllers::{authenticate, activate, signup};
use self::models::User;

const SECRET_KEY: &str = "YOUR SECRET_KEY";

#[derive(Debug, Serialize, Deserialize)]
pub struct JwtBearerClaims {
    pub username: String,
    pub role: Role,
    pub id: i32,
    exp: i64,
}

pub fn encode_token<T: Serialize>(claims: T) -> Result<String, jsonwebtoken::errors::Error> {
    let header = jsonwebtoken::Header::default();
    let key = EncodingKey::from_secret(SECRET_KEY.as_bytes());

    jsonwebtoken::encode(&header, &claims, &key)
}

pub fn decode_token<C: DeserializeOwned>(token: &str) -> Result<jsonwebtoken::TokenData<C>, jsonwebtoken::errors::Error> {
    let key = DecodingKey::from_secret(SECRET_KEY.as_bytes());
    let validation = &Validation::new(Algorithm::HS256);

    jsonwebtoken::decode::<C>(token, &key, &validation)
}

pub fn decode_bearer_token() -> JwtAuth<JwtBearerClaims, ConstDecoder> {
    JwtAuth::new(ConstDecoder::from_secret(SECRET_KEY.as_bytes()))
        .finders(vec![
            Box::new(HeaderFinder::new()),
            Box::new(QueryFinder::new("jwt_token")),
        ])
        .force_passed(true)
}

pub fn create_bearer_token(user: User) -> Result<String, jsonwebtoken::errors::Error> {
    let User { name: username, id, role, .. } = user;
    let exp = OffsetDateTime::now_utc() + Duration::hours(1);

    let claims = JwtBearerClaims {
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
        .push(Router::with_path("activate").post(activate))
}
