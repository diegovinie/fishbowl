use jsonwebtoken::EncodingKey;
use salvo::prelude::*;
use salvo::jwt_auth::{ConstDecoder, QueryFinder, HeaderFinder};
use serde::{Deserialize, Serialize};
use time::{OffsetDateTime, Duration};

const SECRET_KEY: &str = "YOUR SECRET_KEY";

#[derive(Debug, Serialize, Deserialize)]
pub struct JwtClaims {
    username: String,
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
pub fn get_auth() -> String {
    let exp = OffsetDateTime::now_utc() + Duration::minutes(1);

    let claims = JwtClaims {
        username: "gargola".to_string(),
        exp: exp.unix_timestamp(),
    };

    let header = jsonwebtoken::Header::default();
    let key = EncodingKey::from_secret(SECRET_KEY.as_bytes());

    let token = jsonwebtoken::encode(&header, &claims, &key)
        .expect("Error encoding token");

    token
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
