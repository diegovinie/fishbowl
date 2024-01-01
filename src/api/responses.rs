use salvo::prelude::*;
use serde::Serialize;

#[derive(Serialize)]
struct ResourceResponse<T> {
    data: T,
}

#[derive(Serialize)]
struct CollectionResponse<T> {
    data: Vec<T>,
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

pub fn render_resource_created<T: Serialize + Send>(res: &mut Response, resource: T) {
    res.status_code(StatusCode::ACCEPTED);
    res.render(Json(ResourceResponse::<T> { data: resource }));
}

pub fn render_resource_deleted(res: &mut Response, total_deleted: usize) {
    match total_deleted {
        1 => {
            res.status_code(StatusCode::ACCEPTED);
        },
        _other => {
            res.status_code(StatusCode::ACCEPTED);
            res.render(Json(ExecutionResponse {
                message: format!("Total deleted: {}", total_deleted)
            }));
        }
    }
}

pub fn render_resource_updated<T: Serialize + Send>(res: &mut Response, resource: T) {
    res.status_code(StatusCode::ACCEPTED);
    res.render(Json(ResourceResponse::<T> { data: resource }));
}
