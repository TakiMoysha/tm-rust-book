use crate::data_collector::DataCollector;
use crate::message_service::MessageService;
use crate::notification_message_builder::NotificationMessageBuilder;

pub struct MonitoringSystem<D, M, B> {
    data_collector: D,
    message_service: M,
    notification_msg_builder: B,
}

impl<D: DataCollector, M: MessageService, B: NotificationMessageBuilder> MonitoringSystem<D, M, B> {
    pub fn new(data_collector: D, message_service: M, notification_msg_builder: B) -> Self {
        MonitoringSystem {
            data_collector,
            message_service,
            notification_msg_builder,
        }
    }

    pub fn check_alert(&self) {
        let data = self.data_collector.collect_data();

        for d in data {
            if d.contains("2") {
                let msg = self.notification_msg_builder.build_msg(&d);
                self.message_service.send(&msg);
            }
        }
    }
}
