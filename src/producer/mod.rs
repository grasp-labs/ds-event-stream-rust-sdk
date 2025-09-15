//! Kafka producer module.
//!
//! ## Overview
//! A thin, opinionated wrapper around [`rdkafka`]'s [`FutureProducer`].  The
//! goal is to minimise boiler‑plate when publishing JSON‑serialisable payloads
//! to the DS Event Stream Kafka cluster while still giving full control over
//! configuration when needed.
//!
//! Features
//! * Lazy, fallible construction via [`KafkaProducer::new`].
//! * Reads the bootstrap servers from the `KAFKA_BOOTSTRAP_SERVERS` env var.
//! * Emits structured [`tracing`] spans for each send operation.
//! * Transparently maps errors into your project's [`ProducerError`] enum.
//!
//! ### Example
//! ```no_run
//! use ds_event_stream_rust_sdk::producer::KafkaProducer;
//! use ds_event_stream_rust_sdk::model::EventStream;
//! use uuid::Uuid;
//! use chrono::Utc;
//!
//! #[tokio::main]
//! async fn main() -> Result<(), Box<dyn std::error::Error>> {
//!     let producer = KafkaProducer::new("username", "password")?;
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
//!         body: None,
//!         body_uri: None,
//!         metadata: None,
//!         tags: None,
//!     };
//!     producer.send_message("user-created", "user-42", &payload).await?;
//!     Ok(())
//! }
//! ```

use std::{env, time::Duration};

use rdkafka::{
    config::ClientConfig,
    producer::{FutureProducer, FutureRecord},
};
use tracing::{error, info};

use crate::error::ProducerError;
use crate::model::v1::EventStream;

// region: --> KafkaProducer

/// Wrapper around an [`rdkafka::producer::FutureProducer`].
///
/// The producer is configured once and can then be cheaply cloned thanks to
/// the internal `Arc` in *rdkafka*'s handle.
#[derive(Clone)]
pub struct KafkaProducer {
    inner: FutureProducer,
    timeout: Duration,
}

impl KafkaProducer {
    /// Number of milliseconds the producer will wait for an acknowledgment
    /// before treating the send as failed.
    const DEFAULT_SEND_TIMEOUT_MS: u64 = 5_000;

    /// Constructs a new [`KafkaProducer`] using the `KAFKA_BOOTSTRAP_SERVERS`
    /// environment variable.
    ///
    /// # Arguments
    ///
    /// * `username` - The username to use for authentication
    /// * `password` - The password to use for authentication
    ///
    /// # Errors
    /// * [`ProducerError::MissingEnvVar`]   if the env var is not set.
    /// * [`ProducerError::Kafka`]            if the underlying `rdkafka`
    ///   producer fails to initialise (rare — usually wrong config).
    pub fn new(username: &str, password: &str) -> Result<Self, ProducerError> {
        let bootstrap =
            env::var("KAFKA_BOOTSTRAP_SERVERS").map_err(|_| ProducerError::MissingEnvVar {
                var_name: "KAFKA_BOOTSTRAP_SERVERS".to_string(),
            })?;

        let inner: FutureProducer = ClientConfig::new()
            .set("bootstrap.servers", &bootstrap)
            .set(
                "message.timeout.ms",
                Self::DEFAULT_SEND_TIMEOUT_MS.to_string(),
            )
            .set("session.timeout.ms", "6000")
            .set("acks", "all")
            .set("retries", "3")
            .set("compression.type", "snappy")
            .set("batch.size", "16384")
            .set("linger.ms", "5")
            .set("max.in.flight.requests.per.connection", "5")
            .set("request.timeout.ms", "30000")
            .set("delivery.timeout.ms", "120000")
            .set("security.protocol", "SASL_PLAINTEXT")
            .set("sasl.mechanisms", "SCRAM-SHA-512")
            .set("sasl.username", username)
            .set("sasl.password", password)
            .create()
            .map_err(ProducerError::Kafka)?;

        info!(servers = %bootstrap, "Kafka producer initialised");

        Ok(Self {
            inner,
            timeout: Duration::from_millis(Self::DEFAULT_SEND_TIMEOUT_MS),
        })
    }

    /// Sends a key‑ed JSON message to **`topic`**.
    ///
    /// * `T` must implement [`serde::Serialize`] and [`serde::de::DeserializeOwned`].
    /// * `key` is used for partitioning; choose a deterministic key for *exactly
    ///   once‑per‑key* semantics.
    ///
    /// The function is instrumented with [`tracing`]; any error bubbles up as
    /// [`ProducerError`].
    pub async fn send_message(
        &self,
        topic: &str,
        key: &str,
        payload: &EventStream,
    ) -> Result<(), ProducerError> {
        let payload_json = serde_json::to_string(payload).map_err(ProducerError::Json)?;
        let record = FutureRecord::to(topic).payload(&payload_json).key(key);

        match self.inner.send(record, self.timeout).await {
            Ok((partition, offset)) => {
                info!(partition, offset, "message produced to topic: {}", topic);
                Ok(())
            }
            Err((err, _msg)) => {
                error!(error = %err, "failed to produce message to topic: {}", topic);
                Err(ProducerError::Kafka(err))
            }
        }
    }
}

// endregion: --> KafkaProducer

// region: --> Tests

#[cfg(test)]
mod tests {
    use crate::error::ProducerError;

    use super::*;

    #[test]
    fn test_producer_new() {
        std::env::set_var("KAFKA_BOOTSTRAP_SERVERS", "localhost:9092");
        let producer = KafkaProducer::new("username", "password").unwrap();
        assert_eq!(producer.timeout.as_millis(), 5000);
    }

    #[test]
    fn test_producer_missing_bootstrap_servers() {
        // Ensure the env var is definitely not set
        std::env::remove_var("KAFKA_BOOTSTRAP_SERVERS");
        let result = KafkaProducer::new("username", "password");
        assert!(matches!(result, Err(ProducerError::MissingEnvVar { .. })));
        // Clean up after test
        std::env::set_var("KAFKA_BOOTSTRAP_SERVERS", "localhost:9092");
    }

    #[test]
    fn test_producer_empty_credentials() {
        std::env::set_var("KAFKA_BOOTSTRAP_SERVERS", "localhost:9092");
        // Empty credentials should fail because rdkafka validates SASL credentials
        let result = KafkaProducer::new("", "");
        assert!(matches!(result, Err(ProducerError::Kafka(_))));
    }

    #[test]
    fn test_producer_timeout_configuration() {
        std::env::set_var("KAFKA_BOOTSTRAP_SERVERS", "localhost:9092");
        let producer = KafkaProducer::new("username", "password").unwrap();
        assert_eq!(producer.timeout.as_millis(), 5000);
        assert_eq!(producer.timeout.as_secs(), 5);
    }

    #[test]
    fn test_producer_clone() {
        std::env::set_var("KAFKA_BOOTSTRAP_SERVERS", "localhost:9092");
        let producer1 = KafkaProducer::new("username", "password").unwrap();
        let producer2 = producer1.clone();
        assert_eq!(producer1.timeout, producer2.timeout);
    }
}

// endregion: --> Tests
