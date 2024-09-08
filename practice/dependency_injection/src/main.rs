mod configuration_manager;
mod data_collector;
mod message_service;

use chrono::{DateTime, Utc};
use std::cell::OnceCell;

use configuration_manager::ConfigurationManager;
use data_collector::{DataCollector, SimpleDataCollector, SqlDataCollector};
use message_service::SMTPMessageService;

pub struct DependencyContainer {
    configuration_manager: OnceCell<ConfigurationManager>,
}

impl DependencyContainer {
    pub fn new() -> Self {
        Self {
            configuration_manager: OnceCell::new(),
        }
    }

    pub fn datetime(&self) -> DateTime<Utc> {
        Utc::now()
    }

    pub fn data_collector_impl(&self) -> impl DataCollector {
        let configuration_manager = self.configuration_manager();
        self.create_data_collectro_dyn(configuration_manager)
    }

    fn create_data_collectro_dyn(
        &self,
        configuration_manager: ConfigurationManager,
    ) -> Box<dyn DataCollector> {
        if let Some(api_key) = configuration_manager.get_api_key() {
            Box::new(SimpleDataCollector::new(api_key.to_string()))
        } else {
            let conn_string = configuration_manager
                .get_database_conn()
                .expect("No database connection string");
            Box::new(SqlDataCollector::new(conn_string.to_string()))
        }
    }

    fn create_configuration_manager(&self) -> ConfigurationManager {
        ConfigurationManager::new()
    }

    pub fn configuration_manager(&self) -> ConfigurationManager {
        self.configuration_manager
            .get_or_init(|| self.create_configuration_manager())
    }

    fn create_message_service(&self, conf_manager: ConfigurationManager) -> impl MessageService {
        SMTPMessageService::new(
            conf_manager.get_username().to_String(),
            conf_manager.get_pass().to_string(),
        )
    }
}

fn main() {
    println!("!END!");
}
