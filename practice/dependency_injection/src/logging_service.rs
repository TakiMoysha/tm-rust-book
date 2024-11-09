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

pub struct StdoutLogginService {
    alert_id: String,
}

impl StdoutLogginService {
    pub async fn new(alert_id: &str) -> Self {
        sleep(Duration::from_secs(2)).await; // async simulation

        StdoutLogginService {
            alert_id: alert_id.to_string(),
        }
    }
}

impl LoggingService for StdoutLogginService {
    fn log(&self, msg: &str) {
        println!("[Alert {}]: {}", self.alert_id, msg);
    }
}
