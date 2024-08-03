use salvo::handler;

use crate::api::errors::ApiResult;


#[handler]
pub async fn invite_followers() -> ApiResult<()> {
    todo!("Pending")
    // repo.create
}

#[handler]
pub async fn request_follow() -> ApiResult<()> {
    todo!("Pending")
    // repo.create
}

#[handler]
pub async fn activate() -> ApiResult<()> {
    todo!("Pending")
    // rest patch
    // repo.update
}

#[handler]
pub async fn un_follow() -> ApiResult<()> {
    todo!("Pending")
    // repo.delete
}

#[handler]
pub async fn remove_follower() -> ApiResult<()> {
    todo!("Pending")
    // repo.delete
}

