use diesel::{RunQueryDsl, SelectableHelper};

use crate::services::database::{contracts::SponsorRepo, establish_connection};
use crate::schema::sponsors::table as sponsors_table;
use super::models::{NewSponsor, Sponsor};
use diesel::result::Error;

pub struct Repo;

impl SponsorRepo for Repo {
    fn insert(&self, new_sponsor: NewSponsor) -> Result<Sponsor, Error> {
        let conn = &mut establish_connection();

        diesel::insert_into(sponsors_table)
            .values(&new_sponsor)
            .returning(Sponsor::as_returning())
            .get_result(conn)

    }
}