use crate::logging_service::LoggingService;

pub trait Repository {
    fn get_data(&self) -> Vec<String>;
}

pub struct Sqlite {}

impl Sqlite {
    pub fn new() -> Self {
        Self {}
    }

    fn new_logging(logging_service: impl LoggingService) -> Self {
        Self {}
    }
}

impl Repository for Sqlite {
    fn get_data(&self) -> Vec<String> {
        todo!()
    }
}
