use crate::data_collector::DataCollector;
use crate::message_service::MessageService;

pub struct MonitoringSystem<D, M> {
    data_collector: D,
    message_service: M,
}

impl<D: DataCollector, M: MessageService> MonitoringSystem<D, M> {
    pub fn new(data_collector: D, message_service: M) -> Self {
        MonitoringSystem {
            data_collector,
            message_service,
        }
    }

    pub fn check_alert(&self) {
        let data = self.data_collector.collect_data();

        for d in data {
            if d.contains("2") {
                self.message_service.send(&d);
            }
        }
    }
}
