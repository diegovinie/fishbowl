pub mod contracts;
pub mod primary_impl;
// #[cfg(test)] 
pub mod mocks;

use salvo::prelude::*;
use diesel::prelude::*;
use diesel::pg::PgConnection;
use std::env;
use std::sync::Arc;
use contracts::DatabaseService;
use self::primary_impl::DatabaseServiceImpl;

pub struct InjectableServices<D: DatabaseService> {
    pub database: D,
}

#[derive(Clone)]
pub struct ServiceInjector {
    database: Arc<dyn DatabaseService>,
}

impl ServiceInjector {
    pub fn new<D: DatabaseService + 'static>(services: InjectableServices<D>) -> Self {
        Self {
            database: Arc::new(services.database),
        }
    }
}

#[async_trait]
impl Handler for ServiceInjector {
    async fn handle(&self, _req: &mut Request, depot: &mut Depot, _res: &mut Response, _ctrl: &mut FlowCtrl) {
        depot.inject::<Arc<dyn DatabaseService>>(self.database.clone());
    }
}

pub fn establish_connection() -> PgConnection {
    let database_url = env::var("DATABASE_URL")
        .expect("DATABASE_URL must be set");

    PgConnection::establish(&database_url)
        .unwrap_or_else(|_| panic!("Error connecting to {}", database_url))
}

pub fn primary_service_injector() -> ServiceInjector {
    let injectable_services = InjectableServices {
        database: DatabaseServiceImpl
    };

    ServiceInjector::new(injectable_services)
}