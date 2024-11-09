use crate::logging_service::{LoggingService, StdoutLogginService};
use crate::monitorint_system::MonitoringSystem;
use crate::notification_message_builder::{
    DefaultNotificationMessageBuilder, NotificationMessageBuilder,
};
use chrono::{DateTime, Utc};
use std::cell::OnceCell;
use std::rc::Rc;

use crate::configuration_manager::ConfigurationManager;
use crate::data_collector::{ApiDataCollector, DataCollector, SqlDataCollector};
use crate::message_service::{MessageService, SMTPMessageServiceImpl};

pub struct DependencyContainer {
    // We will only want to read the configuration once
    configuration_manager: Rc<OnceCell<ConfigurationManager>>,
    // scope dependencies just use an [OnceCell]
    logging_service: OnceCell<StdoutLogginService>,
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

    fn create_data_collector_dyn<'a, L>(
        &self,
        configuration_manager: &ConfigurationManager,
        fn_logging_service: impl Fn() -> L,
    ) -> Box<dyn DataCollector + 'a>
    where
        L: LoggingService + 'a,
    {
        if let Some(api_key) = configuration_manager.get_api_key() {
            let logging_service = fn_logging_service();
            Box::new(ApiDataCollector::new(api_key.to_string(), logging_service))
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
        self.create_data_collector_dyn(configuration_manager, || self.logging_service())
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
        StdoutLogginService::new(alert_id)
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

    fn create_notification_message_builder(&self) -> impl NotificationMessageBuilder {
        DefaultNotificationMessageBuilder::new()
    }

    pub fn notification_message_builder(&self) -> impl NotificationMessageBuilder {
        self.create_notification_message_builder()
    }

    fn create_monitoring_system(
        &self,
        data_collector: impl DataCollector,
        msg_service: impl MessageService,
        notification_message_builder: impl NotificationMessageBuilder,
    ) -> MonitoringSystem<impl DataCollector, impl MessageService, impl NotificationMessageBuilder>
    {
        MonitoringSystem::new(data_collector, msg_service, notification_message_builder)
    }

    pub fn monitoring_system(
        &self,
    ) -> MonitoringSystem<
        impl DataCollector + '_,
        impl MessageService + '_,
        impl NotificationMessageBuilder + '_,
    > {
        self.create_monitoring_system(
            self.data_collector(),
            self.message_service(),
            self.notification_message_builder(),
        )
    }
}
