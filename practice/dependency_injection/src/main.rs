mod configuration_manager;
mod data_collector;
mod dependency_container;
mod logging_service;
mod message_service;
mod monitorint_system;
mod notification_message_builder;

use std::{thread::sleep, time::Duration};

use dependency_container::DependencyContainer;

#[tokio::main]
async fn main() {
    let dc = DependencyContainer::new();

    for i in 1.. {
        let alert_id = format!("Alert{}", i);

        let dc = dc.new_scope(&alert_id);

        let monitoring_system = dc.monitoring_system().await;
        monitoring_system.check_alert();
        dc.monitoring_system().await.check_alert();

        sleep(Duration::from_secs(5));
    }
}
