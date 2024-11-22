use std::io;

use crate::inmemory_store::{InMemoryStore, Repository};
use rocket::*;
use tokio::{
    task::spawn_blocking,
    time::{sleep, Duration},
};

mod database {}

mod monitoring {
    use rocket::tokio::{sync::Mutex, time::interval};
    use std::{
        sync::Arc,
        time::{Duration, SystemTime},
    };

    use crate::inmemory_store::{InMemoryStore, Repository};

    #[derive(Debug)]
    pub struct HealthWatcher {
        signal: Mutex<Option<String>>,
        store: Mutex<InMemoryStore>,
    }

    impl HealthWatcher {
        pub fn new(signal_p: Option<String>, store_p: InMemoryStore) -> HealthWatcher {
            HealthWatcher {
                signal: Mutex::new(signal_p),
                store: Mutex::new(store_p),
            }
        }

        // async fn

        // check url servie status,
        // if website working - save to db working point
        // if website not working - save to db not working point
        async fn check_site_health(&self, alias: &str, url: &str, signal: &Arc<Mutex<bool>>) {
            let mut interval = interval(Duration::from_secs(5));

            loop {
                interval.tick().await;
                let response = reqwest::get(url).await.unwrap();
                let status_code = response.status();
                let time_point = SystemTime::now();
                let response_text = response.text().await.unwrap();

                println!("save ...");
                self.store
                    .lock()
                    .await
                    .save_health_point(alias.to_string(), time_point, status_code, response_text)
                    .unwrap();

                if *signal.lock().await {
                    println!("error in watcher, break loop");
                    break;
                }
                // if self.signal.lock().await.is_some() {
                //     println!("error in watcher, break loop");
                //     break;
                // }
            }
            println!("End of loop");
        }
    }

    #[cfg(test)]
    mod tests {
        use std::{
            borrow::Borrow,
            cell::{Cell, RefCell},
            rc::Rc,
            sync::Arc,
        };

        use super::*;
        use rocket::tokio::{self, time};

        #[tokio::test]
        async fn should_shared_state_between_threads() {
            let for_thread_value = Arc::new(Mutex::new(0));
            let for_control_value = for_thread_value.clone();

            let thr = tokio::spawn(async move {
                loop {
                    let mut lock = for_thread_value.lock().await;
                    if *lock == 10 {
                        break;
                    }

                    *lock += 3;
                    time::sleep(tokio::time::Duration::from_secs(1)).await;
                }
            });

            println!("wait");
            time::sleep(tokio::time::Duration::from_secs(3)).await;
            println!("mod");
            *for_control_value.lock().await = 7;
            let _ = thr.await;
            println!("stopped");

            let value_lock = for_control_value.lock().await;
            println!("{:?} = {:?}", for_control_value, *value_lock);
        }

        #[tokio::test]
        async fn should_check_website_and_save_data() {
            let store = InMemoryStore::new();

            let row_watcher = HealthWatcher::new(None, store);

            let p_signal = Arc::new(Mutex::new(false));
            let p2_signal = Arc::clone(&p_signal);
            let p_watcher = Arc::new(Mutex::new(row_watcher));
            let m_watcher = Arc::clone(&p_watcher);

            let thr = tokio::spawn(async move {
                (*m_watcher.lock().await)
                    .check_site_health("test_site", "https://reqres.in/api/users/1", &p_signal)
                    .await;
            });

            println!("wait");
            time::sleep(tokio::time::Duration::from_secs(10)).await;
            println!("mod");

            *p2_signal.lock().await = true;
            println!("stoping...");
            let _ = thr.await;

            let points = (*p_watcher.lock().await.store.lock().await)
                .get_health_points_by_alias("test_site".to_string());
            println!("{:?}", points);
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

#[get("/counter")]
pub async fn get_counter() -> String {
    format!("counter")
}

#[post("/api/counts")]
pub async fn post_coutner() -> String {
    format!("counter")
}
