use std::sync::{Arc, Mutex};
use diesel::result::Error;
use fishbowl::api::resources::users::models::{User, NewUser};
use fishbowl::database::contracts;
use super::{MockService, Reporter};

pub struct TestUserRepo {
    pub data: Vec<User>,
    pub reporter: Arc<Mutex<Reporter>>,
}

impl MockService<User> for TestUserRepo {
    fn new(data: Vec<User>, reporter: Arc<Mutex<Reporter>>) -> Self {
        Self { data, reporter }
    }

    fn data(&self) -> Vec<User> {
        self.data.clone()
    }
}

impl contracts::UserRepo for TestUserRepo {
    fn list(&self) -> Result<Vec<User>, Error> {
        let mut reporter = self.reporter.lock().unwrap();

        reporter.report("list".to_string(), "hello baby!".to_string());

        Ok(self.data())
    }

    fn find_user(&self, _id: i32) -> Result<User, Error> {
        todo!()
    }

    fn insert_many(&self, users: Vec<NewUser>) ->Result<usize, Error> {
        Ok(users.len())
    }
}
