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
        rc::Rc,
        sync::Arc,
        time::{Duration, SystemTime},
    };

    use crate::inmemory_store::{InMemoryStore, Repository};

    pub struct HealthWatcher {
        error: Option<String>,
        store: Mutex<InMemoryStore>,
    }

    impl HealthWatcher {
        pub fn new(store_p: InMemoryStore) -> HealthWatcher {
            HealthWatcher {
                error: None,
                store: Mutex::new(store_p),
            }
        }

        // // check url servie status,
        // // if website working - save to db working point
        // // if website not working - save to db not working point
        // async fn check_site_health(&self, alias: &str, url: &str) {
        //     let mut interval = interval(Duration::from_secs(5));
        //
        //     loop {
        //         interval.tick().await;
        //         let response = reqwest::get(url).await.unwrap();
        //         let status_code = response.status();
        //         let time_point = SystemTime::now();
        //         let response_text = response.text().await.unwrap();
        //
        //         println!("save ...");
        //         self.store
        //             .lock()
        //             .await
        //             .save_health_point(alias.to_string(), time_point, status_code, response_text)
        //             .unwrap();
        //
        //         // if *signal {
        //         //     println!("error in watcher, break loop");
        //         //     break;
        //         // }
        //         if self.error.is_some() {
        //             println!("error in watcher, break loop");
        //             break;
        //         }
        //     }
        //     println!("End of loop");
        // }
        //
        // async fn set_error(&mut self, msg: String) {
        //     self.error = Some(msg);
        // }
        //
    }

    trait AtomicWatcher: Send + Sync {
        async fn check_site_health(&self, alias: &str, url: &str);
        async fn set_error(&mut self, msg: String);
    }

    impl AtomicWatcher for HealthWatcher {
        // check url servie status,
        // if website working - save to db working point
        // if website not working - save to db not working point
        async fn check_site_health(&self, alias: &str, url: &str) {
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

                // if *signal {
                //     println!("error in watcher, break loop");
                //     break;
                // }
                if self.error.is_some() {
                    println!("error in watcher, break loop");
                    break;
                }
            }
            println!("End of loop");
        }

        async fn set_error(&mut self, msg: String) {
            self.error = Some(msg);
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
                    if *for_thread_value.lock().await == 10 {
                        break;
                    }

                    *for_thread_value.lock().await += 3;
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

        #[ignore = "wip"]
        #[tokio::test]
        async fn should_check_website() {
            let store = InMemoryStore::new();
            let mut watcher = Arc::new(HealthWatcher::new(store));
            let r_store = Rc::new(&watcher);
            // let watcher_p = Cell::new(&watcher);
            let stop_signal = Arc::new(&mut false);
            let _watcher = Arc::clone(&watcher);

            let thr = tokio::spawn(async move {
                // let _watcher = Arc::clone(&watcher);
                (*watcher)
                    .check_site_health("test_site", "https://reqres.in/api/users/1")
                    .await;
            });

            // wait 60 seconds
            println!("wait ");
            tokio::time::sleep(tokio::time::Duration::from_secs(5)).await;
            println!("wait ");
            // *stop_signal = true;
            // watcher.error = Some("error".to_string());
            // r_store.set_error("error".to_string()).await;
            println!("wait ");
            let _ = thr.await;
            println!("exit");

            // let all_services = r_store.lock().await.get("test_site".to_string()).unwrap();
            // println!("{:?}", all_services)

            // todo!();
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
