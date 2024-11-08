mod configuration_manager;
mod data_collector;
mod logging_service;
mod message_service;

use chrono::{DateTime, Utc};
use std::cell::OnceCell;
use std::rc::Rc;

use configuration_manager::ConfigurationManager;
use data_collector::{ApiDataCollector, DataCollector, SqlDataCollector};
use message_service::{SMTPMessageService, SMTPMessageServiceImpl};

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

    pub fn data_collector_impl(&self) -> impl DataCollector {
        let configuration_manager = self.configuration_manager();
        self.create_data_collector_dyn(configuration_manager)
    }

    fn create_data_collector_dyn(
        &self,
        configuration_manager: &ConfigurationManager,
    ) -> Box<dyn DataCollector> {
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

    pub fn data_collector(&self) -> impl DataCollector {
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
    ) -> impl SMTPMessageService {
        SMTPMessageServiceImpl::new(
            configuration_manager.get_username().to_string(),
            configuration_manager.get_pass().to_string(),
        )
    }

    pub fn message_service(&self) -> impl SMTPMessageService {
        let conf_manager = self.configuration_manager();
        self.create_message_service_dyn(conf_manager)
    }

    fn create_logging_service(&self, alert_id: &str) -> logging_service::StdoutLogginService {
        logging_service::StdoutLogginService::new(alert_id)
    }

    pub fn logging_service(&self) -> &logging_service::StdoutLogginService {
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
    println!("!END!");
}
