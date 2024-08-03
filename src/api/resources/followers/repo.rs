use crate::db::contracts::FollowerRepo;
use diesel::result::Error;
use super::models::{Follower, NewFollower};


pub struct Repo;


impl FollowerRepo for Repo {
    fn insert(&self, new_follower: NewFollower) -> Result<Follower, Error> {
        todo!()
    }

    fn update(&self, follower: &Follower) -> Result<Follower, Error> {
        todo!()
    }

    fn delete(&self, id: i32) -> Result<usize, Error> {
        todo!()
    }
}