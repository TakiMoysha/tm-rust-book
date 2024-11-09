mod configuration_manager;
mod data_collector;
mod dependency_container;
mod logging_service;
mod message_service;
mod monitorint_system;
mod notification_message_builder;

use std::{thread::sleep, time::Duration};

use dependency_container::DependencyContainer;
use monitorint_system::MonitoringSystem;

fn main() {
    let dc = DependencyContainer::new();

    for i in 1.. {
        let alert_id = format!("Alert{}", i);

        let dc = dc.new_scope(&alert_id);

        let data_collector = dc.data_collector();
        let message_service = dc.message_service();
        let msg_builder = dc.notification_message_builder();

        let monitoring_system = MonitoringSystem::new(data_collector, message_service, msg_builder);

        monitoring_system.check_alert();

        sleep(Duration::from_secs(5));
    }
}
