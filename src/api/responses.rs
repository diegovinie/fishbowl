use salvo::prelude::*;
use serde::Serialize;
use super::utils::pagination::Pagination;

#[derive(Serialize)]
struct ResourceResponse<T> {
    data: T,
}

#[derive(Serialize)]
struct CollectionResponse<T> {
    data: Vec<T>,
}

#[derive(Serialize)]
struct CollectionPaginatedResponse<T> {
    data: Vec<T>,
    pagination: Pagination,
}

#[derive(Serialize)]
struct ExecutionResponse {
    message: String,
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
