use std::time::Duration;

use async_trait::async_trait;
use chrono::Utc;
use clap::{Arg, Command};

use crate::{
    config::KafkaConfig,
    kafka::{consumer::EventConsumer, producer::EventProducer},
};
mod kafka;

mod error {
    use thiserror::Error;

    #[derive(Debug, Error)]
    pub enum KafkaError {
        #[error("Failed to create kafka client: {0}")]
        ClientCreationError(String),

        #[error("Failed to send message: {0}")]
        MessageSendError(String),

        #[error("Failed to deliver message: {0}")]
        MessageDeliveryError(String),

        #[error("Failed to deserialize message: {0}")]
        DeserializationError(String),

        #[error("Timeout: {0}")]
        Timeout(String),
    }

    pub type KafkaResult<T> = Result<T, KafkaError>;
}

mod config {
    #[derive(Debug, Clone)]
    pub struct KafkaConfig {
        pub bootstrap_server: String,
        pub topic: String,
        pub group_id: String,
        pub timeout_ms: u64,
        pub max_retries: u32,
    }

    impl Default for KafkaConfig {
        fn default() -> Self {
            Self {
                bootstrap_server: "localhost:9092".to_string(),
                topic: "rust-book.practice.demo".to_string(),
                group_id: "demo-group".to_string(),
                timeout_ms: 5000,
                max_retries: 3,
            }
        }
    }
}

struct MessagePrinter {}

impl MessagePrinter {
    fn new() -> Box<Self> {
        Box::new(MessagePrinter {})
    }
}

#[async_trait]
impl kafka::consumer::MessageHandler for MessagePrinter {
    async fn handle(&self, key: &[u8], payload: &[u8]) -> error::KafkaResult<()> {
        println!(
            "key: {}, payload: {}",
            String::from_utf8_lossy(key),
            String::from_utf8_lossy(payload)
        );
        Ok(())
    }
}

#[tokio::main]
async fn main() {
    let matches = Command::new("demo-kafka")
        .version(option_env!("CARGO_PKG_VERSION").unwrap_or("draft"))
        .about("Demo kafka connection.")
        .arg(
            Arg::new("bootstrap-server")
                .short('b')
                .default_value("localhost:9092")
                .required(false),
        )
        .arg(
            Arg::new("group-id")
                .short('g')
                .default_value("demo-group")
                .required(false),
        )
        .get_matches();

    tracing_subscriber::fmt::init();
    let config = KafkaConfig::default();

    let producer = EventProducer::new(config.clone()).expect("Failed to create producer");
    let consumer =
        EventConsumer::new(config, MessagePrinter::new()).expect("Failed to create consumer");

    tokio::task::spawn(async move {
        let mut interval = tokio::time::interval(Duration::from_secs(1));
        loop {
            tokio::select! {
                _ = interval.tick() => {
                    let key = Utc::now().timestamp().to_string();
                    match producer.send_event(&key, "Hello, world!").await {
                        Ok(_) => {}
                        Err(e) => tracing::error!("Failed to send message: {}", e),
                    }
                }
            }
        }
    });
    consumer.start().await.expect("Failed to start consumer");
}
