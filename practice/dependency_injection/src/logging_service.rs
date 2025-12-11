use std::time::Duration;
use tokio::time::sleep;

pub trait LoggingService {
    fn log(&self, msg: &str);
}

impl<L: LoggingService + ?Sized> LoggingService for &L {
    fn log(&self, msg: &str) {
        (*self).log(msg);
    }
}

pub struct StdoutLoggingService {
    alert_id: String,
}

impl StdoutLoggingService {
    pub async fn new(alert_id: &str) -> Self {
        sleep(Duration::from_secs(2)).await; // async simulation

        StdoutLoggingService {
            alert_id: alert_id.to_string(),
        }
    }
}

impl LoggingService for StdoutLoggingService {
    fn log(&self, msg: &str) {
        println!("[Alert {}]: {}", self.alert_id, msg);
    }
}
