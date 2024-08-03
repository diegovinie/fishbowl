
pub mod contracts {
    use crate::api::resources::users::models::User;

    pub trait Notifier: Send + Sync {
        async fn send(&self, recipient: &User, message: String) -> bool;
    }
}

use contracts::*;
use crate::api::resources::users::models::User;

#[derive(Default)]
pub struct ConsoleNotifier;

impl Notifier for ConsoleNotifier {
    async fn send(&self, recipient: &User, message: String) -> bool {
        println!("for {} <{}>:", recipient.name, recipient.email);
        println!("message: {}", message);

        true
    }
}



