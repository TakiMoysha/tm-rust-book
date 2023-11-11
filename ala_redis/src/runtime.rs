#[macro_use(lazy_static)]

use lazy_static::lazy_static;
use std::sync::Mutex;
use tokio::time::Instant;

// для определения только внутри модуля:
// для уменьшения зависимостей, 
// для увеличения безопасности - недоступно из других модулей
lazy_static! {
    static ref START_TIME: Mutex<Instant> = Mutex::new(Instant::now());
}

pub fn get_timework() { }

