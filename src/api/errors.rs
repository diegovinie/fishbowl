use std::fmt::{Display, Debug};
use salvo::prelude::*;
use serde::Serialize;
use thiserror::Error;
use std::num::{ParseIntError, ParseFloatError};

#[derive(Error, Debug)]
pub enum ApiError {
    #[error("diesel: `{0}`")]
    Diesel(#[from] diesel::result::Error),
    #[error("injection: `{0}`")]
    Injection(InjectionError),
    #[error("parse-int: `{0}`")]
    ParseInt(ParseIntError, String),
    #[error("parse-float: `{0}`")]
    ParseFloat(ParseFloatError, String),
    #[error("file-not-found: `{0}`")]
    FieldNotFound(String),
    #[error("parse-form-data: `{0}`")]
    ParseFormData(#[from] salvo::http::ParseError),
}

#[async_trait]
impl Writer for ApiError {
    async fn write(mut self, _req: &mut Request, _depot: &mut Depot, res: &mut Response) {
        match self {
            ApiError::Injection(_details) => {
                render_injection_error(res, "product_repo");
            },
            ApiError::ParseInt(error, field) => {
                res.status_code(StatusCode::BAD_REQUEST);
                res.render(json(format!("Error parsing `{field}` from the form: {error:?}")));
            },
            ApiError::ParseFloat(error, field) => {
                res.status_code(StatusCode::BAD_REQUEST);
                res.render(json(format!("Error parsing `{field}` from the form: {error:?}")));
            },
            ApiError::FieldNotFound(field) => {
                res.status_code(StatusCode::BAD_REQUEST);
                res.render(json(format!("Field `{field}` not found")));
            },
            ApiError::ParseFormData(error) => {
                res.status_code(StatusCode::BAD_REQUEST);
                res.render(json(format!("Error parsing the form data: {error}")));
            },
            ApiError::Diesel(error) => match error {
                diesel::result::Error::NotFound => {
                    render_resource_not_found(res, "any")
                },
                _ => {
                    res.status_code(StatusCode::INTERNAL_SERVER_ERROR);
                    res.render(json("undefined error".to_string()));
                }
            },
        }
    }
}

// impl From<ParseIntError> for ApiError {
//     fn from(value: ParseIntError) -> Self {
//         Self::ParseInt(value)
//     }
// }

pub type ApiResult<T> = Result<T, ApiError>;

#[derive(Debug)]
pub struct InjectionError;

impl Display for InjectionError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Error getting from depot")
    }
}

impl std::error::Error for InjectionError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        None
    }

    fn cause(&self) -> Option<&dyn std::error::Error> {
        self.source()
    }
}

#[derive(Debug)]
pub enum Error<'a> {
    FieldNotFound(&'a str),
    ParseIntErr(&'a str),
    ParseFloatErr(&'a str),
}

#[derive(Serialize)]
struct ErrorMessage {
    message: String,
}

#[derive(Serialize)]
struct ErrorResponse {
    error: ErrorMessage,
}

fn make_json_response(message: String) -> Json<ErrorResponse> {
    Json(ErrorResponse { error: ErrorMessage { message } })
}

use make_json_response as json;

pub fn render_form_data_error(res: &mut Response, error: impl Display) {
    res.status_code(StatusCode::BAD_REQUEST);
    res.render(json(format!("Error getting the form data: {error}")));
}

pub fn render_resource_not_found(res: &mut Response, resource: impl Display) {
    res.status_code(StatusCode::NOT_FOUND);
    res.render(json(format!("Error `{resource}` not found")));
}

pub fn render_cast_error(res: &mut Response, error: impl Debug) {
    res.status_code(StatusCode::BAD_REQUEST);
    res.render(json(format!("Error parsing the form data fields: {error:?}")));
}

pub fn render_parse_field_error(res: &mut Response, error: impl Display, field: impl Display) {
    res.status_code(StatusCode::BAD_REQUEST);
    res.render(json(format!("Incorrect <{field}>: {error}")));
}

pub fn render_get_user_id_not_found(res: &mut Response) {
    res.status_code(StatusCode::INTERNAL_SERVER_ERROR);
    res.render(json(format!("Couldn't read `user_id` from depot")));
}

pub fn render_auth_validation_none(res: &mut Response) {
    res.status_code(StatusCode::NOT_ACCEPTABLE);
    res.render(json("Authentication failed".to_string()));
}

pub fn render_auth_create_token_error(res: &mut Response, error: impl Debug) {
    res.status_code(StatusCode::INTERNAL_SERVER_ERROR);
    res.render(json(format!("Error creating token: {error:?}")));
}

pub fn render_inconsistency_error(res: &mut Response, value: impl Display) {
    res.status_code(StatusCode::BAD_REQUEST);
    res.render(json(format!("Error: `{value}` is not consistent")));
}

pub fn render_unauthorized(res: &mut Response) {
    res.status_code(StatusCode::FORBIDDEN);
    res.render(json(format!("Not enough privileges")));
}

pub fn render_injection_error(res: &mut Response, value: impl Display) {
    res.status_code(StatusCode::INTERNAL_SERVER_ERROR);
    res.render(json(format!("Error: getting `{value}` from depot")));
}

pub fn render_db_resource_not_associated(res: &mut Response, resource: impl Display) {
    res.status_code(StatusCode::FORBIDDEN);
    res.render(json(format!("Error: `user` not associated to `{resource}`")));
}

pub fn render_db_retrieving_error(res: &mut Response, error: impl Display, resource: impl Display) {
    res.status_code(StatusCode::INTERNAL_SERVER_ERROR);
    res.render(json(format!("Error loading `{resource}`: {error}")));
}

pub fn render_db_insert_error(res: &mut Response, error: impl Display, resource: impl Display) {
    res.status_code(StatusCode::INTERNAL_SERVER_ERROR);
    res.render(json(format!("Error inserting `{resource}`: {error}")));
}

pub fn render_db_update_error(res: &mut Response, error: impl Display, resource: impl Display) {
    res.status_code(StatusCode::INTERNAL_SERVER_ERROR);
    res.render(json(format!("Error updating `{resource}`: {error}")));
}

pub fn render_db_delete_error(res: &mut Response, error: impl Display, resource: impl Display) {
    res.status_code(StatusCode::NOT_FOUND);
    res.render(json(format!("Error deleting `{resource}`: {error}")));
}
