use std::io;

use crate::inmemory_store::{InMemoryStore, Repository};
use rocket::*;
use tokio::{
    task::spawn_blocking,
    time::{sleep, Duration},
};

mod database {}

mod monitoring {
    use std::time::{Duration, SystemTime};

    use chrono::offset::Utc;
    use chrono::DateTime;
    use rocket::tokio::time::interval;

    use crate::inmemory_store::InMemoryStore;

    // check url servie status,
    // if website working - save to db working point
    // if website not working - save to db not working point
    async fn check_site_health(alias: &str, url: &str, store: InMemoryStore) {
        let mut interval = interval(Duration::from_secs(10));

        loop {
            interval.tick().await;
            let response = reqwest::get(url).await.unwrap();
            let status_code = response.status();
            let time_point = SystemTime::now();
            let response_text = response.text().await.unwrap();

            store
                .save_health_point(alias, time_point, status_code, response_text)
                .unwrap();
        }
    }

    #[cfg(test)]
    mod tests {
        use std::sync::Arc;

        use super::*;
        use rocket::tokio::{self, sync::Mutex};

        #[tokio::test]
        async fn should_check_website() {
            let store = Arc::new(Mutex::from(InMemoryStore::new()));
            let thr = std::thread::spawn(move || {
                check_site_health(
                    "test_site",
                    "https://reqres.in/api/users/1".into(),
                    // store.lock(),
                );
            });

            // tokio::time::sleep(tokio::time::Duration::from_secs(60)).await;
            // thr.join().unwrap();
            //
            // let all_services = store.get_health_points_by_alias("test_site").unwrap();
            // println!("{:?}", all_services)
        }
    }
}

#[get("/")]
pub async fn index() -> Option<fs::NamedFile> {
    fs::NamedFile::open("index.html").await.ok()
}

#[get("/webservices")]
pub async fn get_webservices(services_store: &State<InMemoryStore>) -> String {
    let all_services = services_store.all().unwrap();

    let service_list = all_services
        .iter()
        .map(|(name, url)| format!("<li>{} [{}]</li>", name, url))
        .collect::<Vec<String>>()
        .join("");

    format!("<h1>Services</h1><ol>{}</ol>", service_list)
}

#[post("/webservices")]
pub async fn new_webservices() {}

#[get("/delay/<seconds>")]
pub async fn delay(seconds: u64) -> String {
    sleep(Duration::from_secs(seconds)).await;
    format!("{} seconds delayed", seconds)
}

#[get("/blocking_task")]
pub async fn blocking_task() -> io::Result<Vec<u8>> {
    let vec = spawn_blocking(|| std::fs::read("README.md"))
        .await
        .map_err(|e| io::Error::new(io::ErrorKind::Interrupted, e))??;
    Ok(vec)
}
