pub trait DataCollector {
    fn collect_data(&self) -> Vec<String>;
}

pub struct SimpleDataCollector {
    api_key: String,
}

impl SimpleDataCollector {
    pub fn new(api_key: String) -> Self {
        Self { api_key }
    }
}

impl DataCollector for SimpleDataCollector {
    fn collect_data(&self) -> Vec<String> {
        vec!["data1".to_string(), "data2".to_string()]
    }
}

pub struct SqlDataCollector {
    conn_string: String,
}

impl SqlDataCollector {
    pub fn new(conn_string: String) -> Self {
        Self { conn_string }
    }
}

impl DataCollector for SqlDataCollector {
    fn collect_data(&self) -> Vec<String> {
        vec!["data1".to_string(), "data2".to_string()]
    }
}

// упаковываем DataCollector в Box, что бы можно было использовать `impl DataCollector`
impl<T: DataCollector + ?Sized> DataCollector for Box<T> {
    fn collect_data(&self) -> Vec<String> {
        (**self).collect_data()
        // T.collect_data(self)
    }
}
