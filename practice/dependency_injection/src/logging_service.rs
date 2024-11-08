pub trait LoggingService {
    fn log(&self, msg: &str);
}

pub struct StdoutLogginService {
    alert_id: String,
}

impl StdoutLogginService {
    pub fn new(alert_id: &str) -> Self {
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

impl<L: LoggingService + ?Sized> LoggingService for &L {
    fn log(&self, msg: &str) {
        (*self).log(msg);
    }
}
