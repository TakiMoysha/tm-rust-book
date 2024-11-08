use logging_service::LoggingService;

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

// теперь любой DataCollector обернут в Box, что бы мы могли не указывать это явно
// Box<dyn DataCollector> == impl DataCollector
impl<T: DataCollector + ?Sized> DataCollector for Box<T> {
    fn collect_data(&self) -> Vec<String> {
        (**self).collect_data()
        // T.collect_data(self)
    }
}

pub struct ApiDataCollector<L> {
    api_key: String,
    logging_service: L,
}

impl<L: LoggingService> ApiDataCollector<L> {
    pub fn new(api_key: String, logging_service: L) -> Self {
        Self {
            api_key,
            logging_service,
        }
    }
}

impl<L: LoggingService> DataCollector for ApiDataCollector<L>
{
    fn collect_data(&self) -> Vec<String> {
        let data = vec!["data1".to_string(), "data2".to_string()]

        for d in data.iter() {
            self.logging_service.log(&d);
        }

        data
    }
}
