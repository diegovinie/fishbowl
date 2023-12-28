use std::fmt::{Display, Debug};
use salvo::prelude::*;

#[derive(Debug)]
pub enum Error<'a> {
    FieldNotFound(&'a str),
    ParseFloatErr(&'a str),
}

pub fn render_form_data_error(res: &mut Response, error: impl Display) {
    res.status_code(StatusCode::BAD_REQUEST);
    res.render(format!("Error getting the form data: {error}"));
}

pub fn render_resource_not_found(res: &mut Response, resource: impl Display) {
    res.status_code(StatusCode::NOT_FOUND);
    res.render(format!("Error `{resource}` not found"));
}

pub fn render_cast_error(res: &mut Response, error: impl Debug) {
    res.status_code(StatusCode::BAD_REQUEST);
    res.render(format!("Error parsing the form data fields: {error:?}"));
}

pub fn render_parse_field_error(res: &mut Response, error: impl Display, field: impl Display) {
    res.status_code(StatusCode::BAD_REQUEST);
    res.render(format!("Incorrect <{field}>: {error}"));
}

pub fn render_get_user_id_not_found(res: &mut Response) {
    res.status_code(StatusCode::INTERNAL_SERVER_ERROR);
    res.render(format!("Couldn't read `user_id` from depot"));
}

pub fn render_db_retrieving_error(res: &mut Response, error: impl Display, resource: impl Display) {
    res.status_code(StatusCode::INTERNAL_SERVER_ERROR);
    res.render(format!("Error loading `{resource}`: {error}"));
}

pub fn render_db_insert_error(res: &mut Response, error: impl Display, resource: impl Display) {
    res.status_code(StatusCode::INTERNAL_SERVER_ERROR);
    res.render(format!("Error inserting `{resource}`: {error}"));
}

pub fn render_db_update_error(res: &mut Response, error: impl Display, resource: impl Display) {
    res.status_code(StatusCode::INTERNAL_SERVER_ERROR);
    res.render(format!("Error updating `{resource}`: {error}"));
}

pub fn render_db_delete_error(res: &mut Response, error: impl Display, resource: impl Display) {
    res.status_code(StatusCode::INTERNAL_SERVER_ERROR);
    res.render(format!("Error deleting `{resource}`: {error}"));
}
