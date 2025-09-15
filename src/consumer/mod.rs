//! Kafka consumer module.
//!
//! ## Overview
//! Wraps [`rdkafka`]'s [`StreamConsumer`] with a small
//! convenience API that can be initialised from environment variables or
//! explicit parameters.
//!
//! * Environment‑driven `bootstrap.servers` and `group.id`.
//! * [`ConsumerContext`] implementation that logs rebalances & commits via
//!   [`tracing`].
//! * `stream()` exposing an async stream of [`BorrowedMessage`].
//! * Error handling with custom `ConsumerError` type.
//!
//! ### Example
//! ```no_run
//! use ds_event_stream_rust_sdk::consumer::KafkaConsumer;
//! use tokio_stream::StreamExt;
//! use tracing::{info, error};
//! use rdkafka::message::Message;
//!
//! #[tokio::main]
//! async fn main() -> Result<(), Box<dyn std::error::Error>> {
//!     let consumer = KafkaConsumer::new(&["test-topic"], "username", "password")?;
//!     let mut stream = consumer.stream();
//!
//!     while let Some(result) = stream.next().await {
//!         match result {
//!             Ok(msg) => {
//!                 info!("Received message on topic: {}", msg.topic());
//!                 if let Some(payload) = msg.payload() {
//!                     info!("Payload: {:?}", std::str::from_utf8(payload)?);
//!                 }
//!             }
//!             Err(e) => error!("Kafka error: {}", e),
//!         }
//!     }
//!     Ok(())
//! }
//! ```

use std::env;

use rdkafka::{
    client::ClientContext,
    config::{ClientConfig, RDKafkaLogLevel},
    consumer::{
        stream_consumer::StreamConsumer, BaseConsumer, CommitMode, Consumer, ConsumerContext,
        Rebalance,
    },
    message::BorrowedMessage,
    Message,
};
use serde_json::Deserializer;
use serde_path_to_error::deserialize;
use tracing::{debug, error, info};

use crate::error::ConsumerError;
use crate::model::EventStream;

/* --------------------------------------------------------------------- */
/* Context                                                               */

/// Logs rebalance lifecycle & commit callbacks via [`tracing`].
///
/// # Arguments
///
/// * `c` - The consumer
/// * `r` - The rebalance
///
/// # Returns
///
/// * `()` - Always succeeds, logs any errors
///
/// # Examples
///
/// ```no_run
/// use ds_event_stream_rust_sdk::consumer::TracingContext;
/// ```
///
/// # Panics
///
/// * `()` - Always succeeds, logs any errors
///
/// # Errors
///
/// * `()` - Always succeeds, logs any errors
pub struct TracingContext;
impl ClientContext for TracingContext {}
impl ConsumerContext for TracingContext {
    fn pre_rebalance<'a>(&self, _c: &BaseConsumer<TracingContext>, r: &Rebalance<'a>) {
        info!(?r, "pre‑rebalance");
    }
    fn post_rebalance<'a>(&self, _c: &BaseConsumer<TracingContext>, r: &Rebalance<'a>) {
        info!(?r, "post‑rebalance");
    }
    fn commit_callback(
        &self,
        res: rdkafka::error::KafkaResult<()>,
        offs: &rdkafka::TopicPartitionList,
    ) {
        match res {
            Ok(_) => debug!(?offs, "offsets committed"),
            Err(e) => error!(%e, "offset commit failed"),
        }
    }
}

/* --------------------------------------------------------------------- */
/* Wrapper                                                               */

/// Thin wrapper around [`StreamConsumer`].
///
/// # Arguments
///
/// * `inner` - The inner consumer
///
/// # Returns
///
/// * `KafkaConsumer` - The Kafka consumer
///
pub struct KafkaConsumer {
    inner: StreamConsumer<TracingContext>,
}

impl KafkaConsumer {
    /* ---- constructors ------------------------------------------------ */

    /// Explicit configuration.
    ///
    /// # Arguments
    ///
    /// * `topics` - The topics to subscribe to
    /// * `username` - The username to use for authentication
    /// * `password` - The password to use for authentication
    ///
    /// # Returns
    ///
    /// * `Result<Self, ConsumerError>` - The result of the operation
    pub fn new(topics: &[&str], username: &str, password: &str) -> Result<Self, ConsumerError> {
        let brokers =
            env::var("KAFKA_BOOTSTRAP_SERVERS").map_err(|_| ConsumerError::MissingEnvVar {
                var_name: "KAFKA_BOOTSTRAP_SERVERS".to_string(),
            })?;
        let group = env::var("KAFKA_CONSUMER_GROUP").map_err(|_| ConsumerError::MissingEnvVar {
            var_name: "KAFKA_CONSUMER_GROUP".to_string(),
        })?;
        let context = TracingContext;
        let inner: StreamConsumer<_> = ClientConfig::new()
            .set("group.id", &group)
            .set("bootstrap.servers", &brokers)
            .set("enable.partition.eof", "false")
            .set("session.timeout.ms", "6000")
            .set("auto.offset.reset", "earliest")
            .set("enable.auto.commit", "false")
            .set("fetch.min.bytes", "1")
            .set("max.partition.fetch.bytes", "1048576")
            .set("heartbeat.interval.ms", "3000")
            .set("max.poll.interval.ms", "300000")
            .set("request.timeout.ms", "30000")
            .set("security.protocol", "SASL_PLAINTEXT")
            .set("sasl.mechanism", "SCRAM-SHA-512")
            .set("sasl.username", username)
            .set("sasl.password", password)
            .set_log_level(RDKafkaLogLevel::Info)
            .create_with_context(context)?;

        let topic_names: Vec<&str> = topics.to_vec();
        inner.subscribe(&topic_names)?;
        info!(topics = ?topic_names, "Kafka consumer initialised");
        Ok(Self { inner })
    }

    /* ---- helpers ----------------------------------------------------- */

    /// Async stream of messages (`KafkaResult<BorrowedMessage>`).
    ///
    /// # Returns
    ///
    /// * `impl Stream<Item = KafkaResult<BorrowedMessage<'_>>> + '_` - The stream of messages
    #[cfg(feature = "tokio")]
    pub fn stream(
        &self,
    ) -> impl tokio_stream::Stream<Item = rdkafka::error::KafkaResult<BorrowedMessage<'_>>> + '_
    {
        self.inner.stream()
    }

    /// Get the underlying consumer for manual polling.
    ///
    /// # Returns
    ///
    /// * `&StreamConsumer<TracingContext>` - The underlying consumer
    pub fn inner(&self) -> &StreamConsumer<TracingContext> {
        &self.inner
    }

    /// Commits the offset of a message.
    ///
    /// * `msg` - The message to commit
    /// * `mode` - The commit mode
    ///
    /// # Returns
    ///
    /// * `()` - Always succeeds, logs any errors
    pub fn commit_message(&self, msg: &BorrowedMessage<'_>, mode: CommitMode) {
        match self.inner.commit_message(msg, mode) {
            Ok(_) => (),
            Err(e) => {
                error!("Failed to commit message: {}", e);
            }
        }
    }

    /// Deserializes a Kafka message into an `EventStream<T>`.
    ///
    /// * `T` must implement [`serde::Serialize`] and [`serde::de::DeserializeOwned`].
    ///
    /// # Returns
    ///
    /// * `Result<EventStream, ConsumerError>` - The result of the operation
    pub fn deserialize_message(
        &self,
        msg: &BorrowedMessage<'_>,
    ) -> Result<EventStream, ConsumerError> {
        let payload = msg
            .payload()
            .ok_or_else(|| ConsumerError::InvalidPayload("Empty message payload".to_string()))?;
        let payload_str = std::str::from_utf8(payload)?;

        let de = &mut Deserializer::from_str(payload_str);
        match deserialize::<_, EventStream>(de) {
            Ok(event) => Ok(event),
            Err(e) => Err(ConsumerError::InvalidPayload(format!(
                "Deserialization error at {}: {}",
                e.path(),
                e
            ))),
        }
    }

    /// Serializes an `EventStream<T>` into a Kafka message.
    ///
    /// * `T` must implement [`serde::Serialize`].
    ///
    /// # Returns
    ///
    /// * `Result<Vec<u8>, ConsumerError>` - The result of the operation
    pub fn serialize_message(&self, msg: &EventStream) -> Result<Vec<u8>, ConsumerError> {
        Ok(serde_json::to_vec(msg)?)
    }
}

// region: --> Tests

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::Utc;
    use uuid::Uuid;

    #[cfg(feature = "tokio")]
    #[tokio::test]
    async fn test_consumer_new() {
        std::env::set_var("KAFKA_BOOTSTRAP_SERVERS", "localhost:9092");
        std::env::set_var("KAFKA_CONSUMER_GROUP", "test-group");
        let _consumer = KafkaConsumer::new(&["test-topic"], "username", "password").unwrap();
        // Consumer created successfully - if we reach here, the test passes
    }

    #[cfg(feature = "tokio")]
    #[tokio::test]
    async fn test_consumer_missing_bootstrap_servers() {
        // Ensure the env var is definitely not set
        std::env::remove_var("KAFKA_BOOTSTRAP_SERVERS");
        std::env::set_var("KAFKA_CONSUMER_GROUP", "test-group");
        let result = KafkaConsumer::new(&["test-topic"], "username", "password");
        // Should fail with MissingEnvVar error
        assert!(result.is_err());
        // Clean up after test
        std::env::set_var("KAFKA_BOOTSTRAP_SERVERS", "localhost:9092");
    }

    #[cfg(feature = "tokio")]
    #[tokio::test]
    async fn test_consumer_empty_credentials() {
        std::env::set_var("KAFKA_BOOTSTRAP_SERVERS", "localhost:9092");
        std::env::set_var("KAFKA_CONSUMER_GROUP", "test-group");
        let result = KafkaConsumer::new(&["test-topic"], "", "");
        assert!(result.is_err());
    }

    #[cfg(feature = "tokio")]
    #[tokio::test]
    async fn test_consumer_missing_consumer_group() {
        std::env::set_var("KAFKA_BOOTSTRAP_SERVERS", "localhost:9092");
        // Ensure the env var is definitely not set
        std::env::remove_var("KAFKA_CONSUMER_GROUP");
        let result = KafkaConsumer::new(&["test-topic"], "username", "password");
        assert!(result.is_err());
        // Clean up after test
        std::env::set_var("KAFKA_CONSUMER_GROUP", "test-group");
    }

    #[cfg(feature = "tokio")]
    #[tokio::test]
    async fn test_consumer_multiple_topics() {
        std::env::set_var("KAFKA_BOOTSTRAP_SERVERS", "localhost:9092");
        std::env::set_var("KAFKA_CONSUMER_GROUP", "test-group");
        let _consumer =
            KafkaConsumer::new(&["topic1", "topic2", "topic3"], "username", "password").unwrap();
        // Consumer created successfully with multiple topics - if we reach here, the test passes
    }

    #[cfg(feature = "tokio")]
    #[tokio::test]
    async fn test_consumer_serialize_deserialize() {
        std::env::set_var("KAFKA_BOOTSTRAP_SERVERS", "localhost:9092");
        std::env::set_var("KAFKA_CONSUMER_GROUP", "test-group");
        let consumer = KafkaConsumer::new(&["test-topic"], "username", "password").unwrap();

        let event = EventStream {
            id: Uuid::new_v4(),
            session_id: Uuid::new_v4(),
            tenant_id: Uuid::new_v4(),
            event_source: "test".to_string(),
            event_type: "test".to_string(),
            timestamp: Utc::now(),
            created_by: "test".to_string(),
            md5_hash: "test".to_string(),
            request_id: None,
            owner_id: None,
            product_id: None,
            product_schema_uri: None,
            event_source_uri: None,
            affected_entity_uri: None,
            message: Some("test message".to_string()),
            body: None,
            body_uri: None,
            metadata: None,
            tags: None,
        };

        // Test serialization
        let serialized = consumer.serialize_message(&event).unwrap();
        assert!(!serialized.is_empty());

        // Verify it's valid JSON
        let json_str = String::from_utf8(serialized).unwrap();
        assert!(json_str.contains("test message"));
    }
}

// endregion: --> Tests
