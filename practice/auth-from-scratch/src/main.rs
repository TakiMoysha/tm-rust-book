use reqwest::{self, Method, StatusCode, retry};

mod storage {
    use serde::{Serialize, de::DeserializeOwned};

    trait StoreEntry: Serialize + DeserializeOwned {}
    pub struct Entry<T: StoreEntry> {
        key: String,
        data: T,
    }

    pub fn save(key: String, data: String) {
        todo!()
    }

    pub fn load<T: StoreEntry>(key: String) -> Option<T> {
        todo!()
    }
}

mod session {
    use rand;
    pub struct Session {
        id: String,
        secret_hash: Vec<u8>,
        created_at: i64,
    }
    fn generate_secure_random_string() -> String {
        let mut rng = rand::thread_rng();
        let alphabet = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789";
        let mut random_string = String::with_capacity(32);
        for _ in 0..32 {
            random_string.push(alphabet[rng.gen_range(0..alphabet.len())] as char);
        }
        todo!();
    }
}

static HOST: &str = "https://httpbin.org";
const HTTP_USER_AGENT: &str = "reqwest/0.10.8";

fn should_retry_trarnsient_status(status: StatusCode) -> bool {
    status == StatusCode::REQUEST_TIMEOUT
        || status == StatusCode::TOO_MANY_REQUESTS
        || status.is_server_error()
}
fn build_retry_policy(host: &'static str, retry_post: bool) -> retry::Builder {
    reqwest::retry::for_host(host)
        .max_retries_per_request(3)
        .classify_fn(move |req_rep| match (req_rep.method(), req_rep.status()) {
            (&Method::GET | &Method::HEAD, Some(status))
                if should_retry_trarnsient_status(status) =>
            {
                req_rep.retryable()
            }
            (&Method::GET | &Method::HEAD, None) if req_rep.error().is_some() => {
                req_rep.retryable()
            }
            (&Method::POST, Some(status))
                if retry_post && should_retry_trarnsient_status(status) =>
            {
                req_rep.retryable()
            }
            (&Method::POST, None) if retry_post && req_rep.error().is_some() => req_rep.retryable(),
            _ => req_rep.success(),
        })
}

#[tokio::main(flavor = "current_thread")]
async fn main() {
    let client = reqwest::Client::builder()
        .user_agent(HTTP_USER_AGENT)
        .retry(build_retry_policy(HOST, true))
        .build()
        .expect("Failed to build HTTP client");

    let response = client
        .get(HOST)
        .send()
        .await
        .expect("Failed to send request");

    println!("Response: {:#?}", response);
}
