use std::sync::{Arc, Mutex};

use fishbowl::{api::resources::sponsors::models::{NewSponsor, Sponsor}, db::contracts};

use super::{MockService, Reporter};


pub struct TestSponsorRepo {
    pub data: Vec<Sponsor>,
    pub reporter: Arc<Mutex<Reporter>>,
}

impl MockService<Sponsor> for TestSponsorRepo {
    fn new(data: Vec<Sponsor>, reporter: Arc<Mutex<Reporter>>) -> Self {
        Self { data, reporter }
    }

    fn data(&self) -> Vec<Sponsor> {
        self.data.clone()
    }
}

impl contracts::SponsorRepo for TestSponsorRepo {
    fn insert(&self, new_sponsor: NewSponsor) -> Result<Sponsor, diesel::result::Error> {
        let NewSponsor { leader, amount, user_id, wish_id } = new_sponsor;

        self.reporter.lock()
            .expect("Error locking reporter")
            .register_fn_call("sponsor_repo.insert");

        Ok(Sponsor { id: 1, leader, amount, user_id, wish_id })
    }
}

