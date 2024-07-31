use std::sync::{Arc, Mutex};
use fishbowl::db::contracts;
use fishbowl::api::auth::models::User;
use super::{MockService, Reporter};

pub struct TestAuthRepo {
  pub reporter: Arc<Mutex<Reporter>>,
}

impl MockService<()> for TestAuthRepo {
    fn new(_: Vec<()>, reporter: std::sync::Arc<std::sync::Mutex<super::Reporter>>) -> Self {
        Self { reporter }
    }

    fn data(&self) -> Vec<()> {
        todo!()
    }
}

impl contracts::AuthRepo for TestAuthRepo {
    fn validate(&self, _email_candidate: &str, _password_candidate: &str) -> Option<User> {
        self.reporter.lock()
            .expect("Error")
            .register_fn_call("auth_repo.validate");

        todo!("Return an user")
    }

    fn activate(&self, _user_id: i32, _user_email: &str) -> Result<usize, diesel::result::Error> {
        todo!()
    }
}