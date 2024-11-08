pub trait SMTPMessageService {
    fn send(&self, message: &str);
}

pub struct SMTPMessageServiceImpl {
    username: String,
    password: String,
}

impl SMTPMessageServiceImpl {
    pub fn new(username: String, password: String) -> Self {
        SMTPMessageServiceImpl { username, password }
    }
}

impl SMTPMessageService for SMTPMessageServiceImpl {
    fn send(&self, message: &str) {
        println!("Sending message: {}", message);
    }
}
