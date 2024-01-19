use salvo::prelude::*;
use serde::{Serialize, Deserialize};
use super::utils::pagination::Pagination;
use crate::api::resources::users::models::User;

#[derive(Serialize, Deserialize)]
pub struct ResourceResponse<T> {
    pub data: T,
}

#[derive(Serialize, Deserialize)]
pub struct CollectionResponse<T> {
    pub data: Vec<T>,
}

#[derive(Serialize, Deserialize)]
pub struct CollectionPaginatedResponse<T> {
    pub data: Vec<T>,
    pub pagination: Pagination,
}

#[derive(Serialize, Deserialize)]
pub struct ExecutionResponse {
    pub message: String,
}

#[derive(Serialize)]
struct AuthenticationResponse {
    auth_token: String,
    user: User,
}

pub fn render_resource<T: Serialize + Send>(res: &mut Response, resource: T) {
    res.render(Json(ResourceResponse::<T> { data: resource }));
}

pub fn render_collection<T: Serialize + Send>(res: &mut Response, collection: Vec<T>) {
    res.render(Json(CollectionResponse::<T> { data: collection }));
}

pub fn render_collection_paginated<T: Serialize + Send>(res: &mut Response, collection: Vec<T>, pagination: Pagination)  {
    res.render(Json(CollectionPaginatedResponse::<T> { data: collection, pagination }));
}

pub fn render_resource_created<T: Serialize + Send>(res: &mut Response, resource: T) {
    res.status_code(StatusCode::ACCEPTED);
    res.render(Json(ResourceResponse::<T> { data: resource }));
}

pub fn render_db_execution(res: &mut Response, total: usize) {
    match total {
        1 => {
            res.status_code(StatusCode::ACCEPTED);
        },
        _other => {
            res.status_code(StatusCode::ACCEPTED);
            res.render(Json(ExecutionResponse {
                message: format!("Total row affected: {}", total)
            }));
        }
    }
}

pub fn render_resource_updated<T: Serialize + Send>(res: &mut Response, resource: T) {
    res.status_code(StatusCode::ACCEPTED);
    res.render(Json(ResourceResponse::<T> { data: resource }));
}

pub fn render_authentication(res: &mut Response, user: User, auth_token: String) {
    res.status_code(StatusCode::ACCEPTED);
    res.render(Json(AuthenticationResponse { auth_token, user }));
}