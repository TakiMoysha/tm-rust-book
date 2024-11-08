mod configuration_manager;
mod data_collector;
mod logging_service;
mod message_service;
mod monitorint_system;

use chrono::{DateTime, Utc};
use logging_service::{LoggingService, StdoutLogginService};
use monitorint_system::MonitoringSystem;
use std::time::Duration;
use std::{cell::OnceCell, thread::sleep};
use std::rc::Rc;

use configuration_manager::ConfigurationManager;
use data_collector::{ApiDataCollector, DataCollector, SqlDataCollector};
use message_service::{MessageService, SMTPMessageServiceImpl};

pub struct DependencyContainer {
    // We will only want to read the configuration once
    configuration_manager: Rc<OnceCell<ConfigurationManager>>,
    // scope dependencies just use an [OnceCell]
    logging_service: OnceCell<logging_service::StdoutLogginService>,
    alert_id: String,
}

impl DependencyContainer {
    pub fn new() -> Self {
        Self {
            configuration_manager: Rc::new(OnceCell::new()),
            logging_service: OnceCell::new(),
            alert_id: String::new(),
        }
    }

    pub fn datetime(&self) -> DateTime<Utc> {
        Utc::now()
    }

    pub fn data_collector_impl(&self) -> impl DataCollector + '_ {
        let configuration_manager = self.configuration_manager();
        self.create_data_collector_dyn(configuration_manager)
    }

    fn create_data_collector_dyn(
        &self,
        configuration_manager: &ConfigurationManager,
    ) -> Box<dyn DataCollector + '_> {
        if let Some(api_key) = configuration_manager.get_api_key() {
            Box::new(ApiDataCollector::new(
                api_key.to_string(),
                self.logging_service(),
            ))
        } else {
            Box::new(SqlDataCollector::new(
                configuration_manager
                    .get_database_conn()
                    .expect("No database connection string")
                    .to_string(),
            ))
        }
    }

    pub fn data_collector(&self) -> impl DataCollector + '_ {
        let configuration_manager = self.configuration_manager();
        self.create_data_collector_dyn(configuration_manager)
    }

    fn create_configuration_manager(&self) -> ConfigurationManager {
        ConfigurationManager::new()
    }

    pub fn configuration_manager(&self) -> &ConfigurationManager {
        self.configuration_manager
            .get_or_init(|| self.create_configuration_manager())
    }

    // This method is SingletonDependency. Get a [MessageService] from the [ConfigurationManager]
    fn create_message_service_dyn(
        &self,
        configuration_manager: &ConfigurationManager,
    ) -> impl MessageService {
        SMTPMessageServiceImpl::new(
            configuration_manager.get_username().to_string(),
            configuration_manager.get_pass().to_string(),
        )
    }

    pub fn message_service(&self) -> impl MessageService {
        let conf_manager = self.configuration_manager();
        self.create_message_service_dyn(conf_manager)
    }

    fn create_logging_service(&self, alert_id: &str) -> StdoutLogginService {
        logging_service::StdoutLogginService::new(alert_id)
    }

    // LoggingService ('_) has the same lifetime as the DependencyContainer (&self)
    pub fn logging_service(&self) -> impl LoggingService + '_ {
        self.logging_service
            .get_or_init(|| self.create_logging_service(&self.alert_id))
    }

    pub fn new_scope(&self, alert_id: &str) -> Self {
        Self {
            // clone the configuration manager
            configuration_manager: self.configuration_manager.clone(),
            // reset the logging service
            logging_service: OnceCell::new(),
            alert_id: alert_id.to_string(),
        }
    }
}

fn main() {
    let dc = DependencyContainer::new();

    for i in 1.. {
        let alert_id = format!("Alert{}", i);

        let dc = dc.new_scope(&alert_id);
        let data_collector = dc.data_collector();
        let message_service = dc.message_service();

        let monitoring_system = MonitoringSystem::new(data_collector, message_service);

        monitoring_system.check_alert();

        sleep(Duration::from_secs(5));
    }
}
