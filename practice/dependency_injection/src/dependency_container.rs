use async_once_cell::OnceCell as AsyncOnceCell;
use chrono::{DateTime, Utc};
use std::{cell::OnceCell, future::Future, rc::Rc};

use crate::configuration_manager::ConfigurationManager;
use crate::data_collector::{ApiDataCollector, DataCollector, SqlDataCollector};
use crate::logging_service::{LoggingService, StdoutLogginService};
use crate::message_service::{MessageService, SMTPMessageServiceImpl};
use crate::monitorint_system::MonitoringSystem;
use crate::notification_message_builder::{
    DefaultNotificationMessageBuilder, NotificationMessageBuilder,
};
use crate::types::{AnotherSpecificType, SpecificType, TraitBasedType};

pub struct DependencyContainer {
    // We will only want to read the configuration once
    configuration_manager: Rc<OnceCell<ConfigurationManager>>,
    // scope dependencies just use an [OnceCell]
    logging_service: AsyncOnceCell<StdoutLogginService>,
    alert_id: String,
}

impl DependencyContainer {
    pub fn new(conf_manager: Option<ConfigurationManager>) -> Self {
        Self {
            configuration_manager: Rc::new(OnceCell::from(conf_manager.unwrap_or_default())),
            logging_service: AsyncOnceCell::new(),
            alert_id: Default::default(),
        }
    }

    // ######################## UTILS ############################
    pub fn new_scope(&self, alert_id: &str, config: Option<ConfigurationManager>) -> Self {
        Self {
            // clone the configuration manager
            configuration_manager: Rc::new(OnceCell::from(config.unwrap_or_default())),
            // reset the logging service
            logging_service: AsyncOnceCell::new(),
            alert_id: alert_id.to_string(),
        }
    }

    #[allow(unused)]
    pub fn datetime(&self) -> DateTime<Utc> {
        Utc::now()
    }

    // ######################## DI-SUPPORT ########################
    async fn create_data_collector<'a, F, L>(
        &self,
        configuration_manager: &ConfigurationManager,
        fn_logging_service: impl Fn() -> F,
    ) -> Box<dyn DataCollector + 'a>
    where
        F: Future<Output = L>,
        L: LoggingService + 'a,
    {
        if let Some(api_key) = configuration_manager.get_api_key() {
            let logging_service = fn_logging_service().await;
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

    pub async fn data_collector(&self) -> impl DataCollector + '_ {
        let configuration_manager = self.configuration_manager();
        self.create_data_collector(configuration_manager, || self.logging_service())
            .await
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

    async fn create_logging_service(&self, alert_id: &str) -> StdoutLogginService {
        StdoutLogginService::new(alert_id).await
    }

    // LoggingService ('_) has the same lifetime as the DependencyContainer (&self)
    pub async fn logging_service(&self) -> impl LoggingService + '_ {
        self.logging_service
            .get_or_init(self.create_logging_service(&self.alert_id))
            .await
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

    pub async fn monitoring_system(
        &self,
    ) -> MonitoringSystem<
        impl DataCollector + '_,
        impl MessageService + '_,
        impl NotificationMessageBuilder + '_,
    > {
        self.create_monitoring_system(
            self.data_collector().await,
            self.message_service(),
            self.notification_message_builder(),
        )
    }

    // ################### MOREOVER DI ################
    #[allow(unused)]
    fn create_trait_based_type(&self) -> impl TraitBasedType {
        SpecificType::new()
    }

    #[allow(unused)]
    pub fn trait_based_type(&self) -> impl TraitBasedType {
        self.create_trait_based_type()
    }

    #[allow(unused)]
    fn create_specific_type(&self) -> SpecificType {
        SpecificType::new()
    }

    #[allow(unused)]
    pub fn specific_type(&self) -> SpecificType {
        self.create_specific_type()
    }

    // dynamic dispatching - needed when type is unknown on compile time and was unveild at runtime
    #[allow(unused)]
    fn create_dynamic_abstract(&self, arg: i32) -> Box<dyn TraitBasedType> {
        if arg > 1 {
            Box::new(SpecificType::new())
        } else {
            Box::new(AnotherSpecificType::new(None))
        }
    }

    #[allow(unused)]
    fn dynamic_abstract(&self) -> impl TraitBasedType {
        let arg = 0;

        self.create_trait_based_type()
    }

    // chaining dependencies - a deps requires some other deps or configuration
    #[allow(unused)]
    fn create_chaining_dep(
        &self,
        dep1: SpecificType,
        dep2: impl TraitBasedType,
    ) -> impl TraitBasedType {
        SpecificType::new()
    }

    #[allow(unused)]
    pub fn chaining_dep(&self) -> impl TraitBasedType {
        self.create_chaining_dep(SpecificType::new(), AnotherSpecificType::new(Some(0)))
    }

    // async dependency
    #[allow(unused)]
    async fn create_async_service(
        &self,
        spec_dep: SpecificType,
        trait_dep: impl TraitBasedType,
    ) -> impl TraitBasedType {
        SpecificType::new()
    }

    #[allow(unused)]
    pub async fn impl_service(&self) -> impl TraitBasedType {
        self.create_async_service(self.specific_type(), self.create_trait_based_type())
            .await
    }
}
