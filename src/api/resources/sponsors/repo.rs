use crate::{services::database::contracts::SponsorRepo, api::errors::ApiResult};

use super::models::{NewSponsor, Sponsor};

pub struct Repo;

impl SponsorRepo for Repo {
    fn insert(&self, new_sponsor: NewSponsor) -> ApiResult<Sponsor> {
        todo!("insert {}", new_sponsor.user_id)
    }
}