pub trait NotificationMessageBuilder {
    fn build_msg(&self, alert: &str) -> String;
}

pub struct DefaultNotificationMessageBuilder;

impl DefaultNotificationMessageBuilder {
    pub fn new() -> Self {
        DefaultNotificationMessageBuilder
    }
}

impl NotificationMessageBuilder for DefaultNotificationMessageBuilder {
    fn build_msg(&self, alert: &str) -> String {
        format!("Alert Notification: {}", alert)
    }
}
