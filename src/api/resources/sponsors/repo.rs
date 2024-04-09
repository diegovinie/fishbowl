use diesel::query_dsl::methods::{FilterDsl, SelectDsl};
use diesel::{ExpressionMethods, RunQueryDsl, SelectableHelper};

use crate::schema;
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
    
    fn list_by_wish(&self, wish_id: i32) -> Result<Vec<Sponsor>, Error> {
        let conn = &mut establish_connection();

        sponsors_table
            .filter(schema::sponsors::wish_id.eq(wish_id))
            .select(Sponsor::as_select())
            .load(conn)
    }
}