use diesel::result::Error;
use fishbowl::api::resources::users::models::{User, NewUser};
use fishbowl::database::contracts;
use super::MockService;

pub struct TestUserRepo {
    pub data: Vec<User>,
}

impl MockService<User> for TestUserRepo {
    fn new(data: Vec<User>) -> Self {
        Self { data }
    }

    fn data(&self) -> Vec<User> {
        self.data.clone()
    }
}

impl contracts::UserRepo for TestUserRepo {
    fn list(&self) -> Result<Vec<User>, Error> {
        Ok(self.data())
    }

    fn find_user(&self, _id: i32) -> Result<User, Error> {
        todo!()
    }

    fn insert_many(&self, users: Vec<NewUser>) ->Result<usize, Error> {
        Ok(users.len())
    }
}
