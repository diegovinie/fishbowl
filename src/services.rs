pub mod primary;
pub mod database;
pub mod notifications;

use salvo::prelude::*;
use std::sync::Arc;
use database::contracts::DatabaseService;
use notifications::contracts::Notifier;

pub struct InjectableServices<D: DatabaseService, N: Notifier> {
    pub database: D,
    pub notifier: N,
}

#[derive(Clone)]
pub struct ServiceInjector {
    database: Arc<dyn DatabaseService>,
    notifier: Arc<dyn Notifier>,
}

impl ServiceInjector {
    pub fn new<D: DatabaseService + 'static, N: Notifier + 'static>(services: InjectableServices<D, N>) -> Self {
        Self {
            database: Arc::new(services.database),
            notifier: Arc::new(services.notifier),
        }
    }
}

#[async_trait]
impl Handler for ServiceInjector {
    async fn handle(&self, _req: &mut Request, depot: &mut Depot, _res: &mut Response, _ctrl: &mut FlowCtrl) {
        depot.inject::<Arc<dyn DatabaseService>>(self.database.clone());
        depot.inject::<Arc<dyn Notifier>>(self.notifier.clone());
    }
}


