use super::notifications::ConsoleNotifier;
use super::database::MainDatabase;
use super::{InjectableServices, ServiceInjector};

pub fn service_injector() -> ServiceInjector {
    let injectable_services = InjectableServices {
        database: MainDatabase,
        notifier: ConsoleNotifier,
    };

    ServiceInjector::new(injectable_services)
}