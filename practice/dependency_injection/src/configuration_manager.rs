pub struct ConfigurationManager {
    smtp_username: String,
    smtp_password: String,
    database_conn: Option<String>,
    api_key: Option<String>,
}

impl ConfigurationManager {
    pub fn new() -> Self {
        Self {
            smtp_username: "user".to_string(),
            smtp_password: "pass".to_string(),
            database_conn: None,
            api_key: Some("key".to_string()),
        }
    }

    pub fn get_username(&self) -> &str {
        &self.smtp_username
    }

    pub fn get_pass(&self) -> &str {
        &self.smtp_password
    }

    pub fn get_database_conn(&self) -> Option<&str> {
        self.database_conn.as_ref().map(|s| s.as_str())
    }

    pub fn get_api_key(&self) -> Option<&str> {
        self.api_key.as_ref().map(|s| s.as_str())
    }
}

