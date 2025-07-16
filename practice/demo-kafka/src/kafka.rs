pub mod producer {
    use std::time::Duration;

    use rdkafka::ClientConfig;
    use rdkafka::producer::{FutureProducer, FutureRecord};

    use crate::config::KafkaConfig;
    use crate::error::{KafkaError, KafkaResult};

    pub struct EventProducer {
        producer: FutureProducer,
        topic: String,
        timeout: Duration,
    }

    impl EventProducer {
        pub fn new(config: KafkaConfig) -> KafkaResult<Self> {
            let producer: FutureProducer = ClientConfig::new()
                .set("bootstrap.servers", &config.bootstrap_server)
                .set("message.timeout.ms", config.timeout_ms.to_string())
                .set("compression.type", "gzip")
                .set("retry.backoff.ms", "500")
                .set("request.required.acks", "all")
                .set("queue.buffering.max.messages", "10000")
                .create()
                .map_err(|e| KafkaError::ClientCreationError(e.to_string()))?;

            Ok(EventProducer {
                producer,
                topic: config.topic,
                timeout: Duration::from_secs(config.timeout_ms / 1000),
            })
        }

        pub async fn send_event<K, V>(&self, key: K, payload: V) -> KafkaResult<()>
        where
            K: AsRef<[u8]>,
            V: AsRef<[u8]>,
        {
            let record = FutureRecord::to(&self.topic)
                .payload(payload.as_ref())
                .key(key.as_ref());

            self.producer
                .send(record, self.timeout)
                .await
                .map_err(|(err, _)| KafkaError::MessageSendError(err.to_string()))?;

            Ok(())
        }
    }
}

pub mod consumer {
    use std::time::Duration;

    use async_trait::async_trait;
    use rdkafka::{
        ClientConfig,
        consumer::{CommitMode, Consumer, StreamConsumer},
    };

    use crate::config::KafkaConfig;
    use crate::error::{KafkaError, KafkaResult};

    #[async_trait]
    pub trait MessageHandler: Send + Sync {
        async fn handle(&self, key: &[u8], payload: &[u8]) -> KafkaResult<()>;
    }

    pub struct EventConsumer {
        consumer: StreamConsumer,
        handler: Box<dyn MessageHandler>,
        max_retries: u32,
    }

    impl EventConsumer {
        pub fn new(config: KafkaConfig, handler: Box<dyn MessageHandler>) -> KafkaResult<Self> {
            let consumer: StreamConsumer = ClientConfig::new()
                .set("bootstrap.servers", &config.bootstrap_server)
                .set("group.id", &config.group_id)
                .set("enable.auto.commit", "false")
                .set("auto.offset.reset", "earliest")
                .set("session.timeout.ms", "6000")
                .set("max.poll.interval.ms", "30000")
                .create()
                .map_err(|e| KafkaError::ClientCreationError(e.to_string()))?;

            consumer
                .subscribe(&[&config.topic])
                .map_err(|e| KafkaError::ClientCreationError(e.to_string()))?;

            Ok(EventConsumer {
                consumer,
                handler,
                max_retries: config.max_retries,
            })
        }

        async fn process_with_retry(&self, key: &[u8], payload: &[u8]) -> KafkaResult<()> {
            let mut retries = 0;
            let mut backoff = Duration::from_millis(100);

            while retries < self.max_retries {
                match self.handler.handle(key, payload).await {
                    Ok(_) => return Ok(()),
                    Err(e) => {
                        tracing::warn!("Retry {} failed: {}", retries, e);
                        retries += 1;
                        tokio::time::sleep(backoff).await;
                        backoff *= 2;
                    }
                }
            }

            Err(KafkaError::MessageDeliveryError(
                "Max retries exceeded".to_string(),
            ))
        }

        pub async fn start(&self) -> KafkaResult<()> {
            while let msg_res = self.consumer.recv().await {
                if msg_res.is_err() {
                    continue;
                }

                match msg_res {
                    Ok(msg) => {
                        // let key = msg.key().unwrap_or_default();
                        // let payload = msg.payload().unwrap_or_default();
                        todo!();
                    },
                    Err(e) => {
                        tracing::error!("Failed to process message: {}", e);
                        tokio::time::sleep(Duration::from_secs(1)).await;
                    }
                }
            }

            // let mut msg_stream = self.consumer.stream();
            //
            // while let Some(msg_result) = msg_stream.next().await {
            //     match msg_result {
            //         Ok(msg) => {
            //             let key = msg.key().unwrap_or_default();
            //             let payload = msg.payload().unwrap_or_default();
            //
            //             match self.process_with_retry(key, payload).await {
            //                 Ok(_) => self
            //                     .consumer
            //                     .commit_message(&msg, CommitMode::Async)
            //                     .map_err(|e| KafkaError::MessageDeliveryError(e.to_string()))?,
            //                 Err(e) => {
            //                     tracing::error!("Failed to process message: {}", e);
            //                     //
            //                 }
            //             }
            //         }
            //         Err(e) => {
            //             tracing::error!("Failed to process message: {}", e);
            //             tokio::time::sleep(Duration::from_secs(1)).await;
            //         }
            //     }
            // }

            Ok(())
        }
    }
}
