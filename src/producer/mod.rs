//! Kafka producer module.
//!
//! ## Overview
//! A thin, opinionated wrapper around [`rdkafka`]'s [`FutureProducer`].  The
//! goal is to minimise boiler‑plate when publishing JSON‑serialisable payloads
//! to the DS Event Stream Kafka cluster while still giving full control over
//! configuration when needed.
//!
//! Features
//! * Lazy, fallible construction via [`KafkaProducer::default`].
//! * Reads the bootstrap servers from the `KAFKA_BOOTSTRAP_SERVERS` env var.
//! * Emits structured [`tracing`] spans for each send operation.
//! * Transparently maps errors into your project's [`ProducerError`] enum.
//!
//! ### Example
//! ```no_run
//! use ds_event_stream_rs_sdk::producer::KafkaProducer;
//! use ds_event_stream_rs_sdk::model::EventStream;
//! use ds_event_stream_rs_sdk::model::topics::Topic;
//! use uuid::Uuid;
//! use chrono::Utc;
//!
//! #[tokio::main]
//! async fn main() -> Result<(), Box<dyn std::error::Error>> {
//!     let producer = KafkaProducer::default("username", "password")?;
//!     let payload = EventStream {
//!         id: Uuid::new_v4(),
//!         session_id: Uuid::new_v4(),
//!         tenant_id: Uuid::new_v4(),
//!         event_source: "test".to_string(),
//!         event_type: "test".to_string(),
//!         timestamp: Utc::now(),
//!         created_by: "test".to_string(),
//!         md5_hash: "test".to_string(),
//!         request_id: None,
//!         owner_id: None,
//!         product_id: None,
//!         product_schema_uri: None,
//!         event_source_uri: None,
//!         affected_entity_uri: None,
//!         message: Some("hello".to_string()),
//!         payload: None,
//!         payload_uri: None,
//!         context: None,
//!         context_uri: None,
//!         metadata: None,
//!         tags: None,
//!     };
//!     producer.send_message(&Topic::DsPipelineJobRequested, "user-42", &payload, None).await?;
//!     Ok(())
//! }
//! ```

use std::env;
use std::time::Duration;

use rdkafka::{
    config::ClientConfig,
    producer::{FutureProducer, FutureRecord},
};
use tracing::{error, info};

use crate::error::ProducerError;
use crate::model::topics::Topic;
use crate::model::v1::EventStream;

// region: --> KafkaProducer

/// Wrapper around an [`rdkafka::producer::FutureProducer`].
///
/// The producer is configured once and can then be cheaply cloned thanks to
/// the internal `Arc` in *rdkafka*'s handle.
#[derive(Clone)]
pub struct KafkaProducer {
    inner: FutureProducer,
}

impl KafkaProducer {
    /// Explicit configuration.
    ///
    /// # Arguments
    ///
    /// * `username` - The username to use for authentication
    /// * `password` - The password to use for authentication
    ///
    /// # Returns
    ///
    /// * `Result<Self, ProducerError>` - The result of the operation
    ///
    /// # Errors
    /// * [`ProducerError::Kafka`]            if the underlying `rdkafka`
    ///
    pub fn new(config: ClientConfig) -> Result<Self, ProducerError> {
        let inner: FutureProducer = config.create().map_err(ProducerError::Kafka)?;
        Ok(Self { inner })
    }

    /// Default configuration.
    ///
    /// # Arguments
    ///
    /// * `username` - The username to use for authentication
    /// * `password` - The password to use for authentication
    ///
    /// # Returns
    ///
    /// * `Result<Self, ProducerError>` - The result of the operation
    ///
    /// # Errors
    ///
    /// * [`ProducerError::MissingEnvVar`]   if the env var is not set.
    /// * [`ProducerError::Kafka`]           if the underlying `rdkafka`
    ///
    pub fn default(username: &str, password: &str) -> Result<Self, ProducerError> {
        let bootstrap = env::var("KAFKA_BOOTSTRAP_SERVERS").map_err(|_| ProducerError::MissingEnvVar {
            var_name: "KAFKA_BOOTSTRAP_SERVERS".to_string(),
        })?;

        let inner: FutureProducer = ClientConfig::new()
            .set("bootstrap.servers", &bootstrap)
            .set("acks", "all")
            .set("retries", "3")
            .set("delivery.timeout.ms", "120000")
            .set("request.timeout.ms", "30000")
            .set("message.timeout.ms", "5000")
            .set("compression.type", "snappy")
            .set("batch.size", "16384")
            .set("linger.ms", "5")
            .set("max.in.flight.requests.per.connection", "5")
            .set("security.protocol", "SASL_PLAINTEXT")
            .set("sasl.mechanisms", "SCRAM-SHA-512")
            .set("sasl.username", username)
            .set("sasl.password", password)
            .create()
            .map_err(ProducerError::Kafka)?;

        info!(servers = %bootstrap, "Kafka producer initialised");
        Ok(Self { inner })
    }

    /// Sends a key‑ed JSON message to **`topic`**.
    ///
    /// * `payload` must be an [`EventStream`] object that implements [`serde::Serialize`].
    /// * `key` is used for partitioning; choose a deterministic key for *exactly
    ///   once‑per‑key* semantics.
    /// * `queue_timeout` is optional; defaults to 5000ms if not provided.
    ///
    /// The function is instrumented with [`tracing`]; any error bubbles up as
    /// [`ProducerError`].
    pub async fn send_message(
        &self,
        topic: &Topic,
        key: &str,
        payload: &EventStream,
        queue_timeout: Option<Duration>,
    ) -> Result<(), ProducerError> {
        let topic_name = topic.name();
        let payload_json = serde_json::to_string(payload).map_err(ProducerError::Json)?;

        let record = FutureRecord::to(&topic_name).payload(&payload_json).key(key);
        let timeout = queue_timeout.unwrap_or(Duration::from_millis(5000));

        match self.inner.send(record, timeout).await {
            Ok(delivery) => {
                info!(
                    partition = delivery.partition,
                    offset = delivery.offset,
                    "message produced to topic: {}",
                    topic_name
                );
                Ok(())
            }
            Err((err, _msg)) => {
                error!(error = %err, "failed to produce message to topic: {}", topic_name);
                Err(ProducerError::Kafka(err))
            }
        }
    }
}

// endregion: --> KafkaProducer
