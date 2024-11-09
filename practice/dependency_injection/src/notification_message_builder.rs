pub trait NotificationMessageBuilder {
    fn build_msg(&self, alert: &str) -> String;
}

pub struct DefaultNotificationMessageBuilder;

impl DefaultNotificationMessageBuilder {
    pub fn new() -> Self {
        DefaultNotificationMessageBuilder
    }
}

pub struct MainNotificationMessageBuilder;

impl MainNotificationMessageBuilder {
    pub fn new() -> Self {
        MainNotificationMessageBuilder
    }
}

impl NotificationMessageBuilder for DefaultNotificationMessageBuilder {
    fn build_msg(&self, alert: &str) -> String {
        format!("Alert Notification: {}", alert)
    }
}

impl NotificationMessageBuilder for MainNotificationMessageBuilder {
    fn build_msg(&self, alert: &str) -> String {
        format!("[proc-0]Alert: {}", alert)
    }
}
