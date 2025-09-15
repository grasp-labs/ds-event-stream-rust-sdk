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
//! use ds_event_stream_rust_sdk::{KafkaConsumer};
//! use tokio_stream::StreamExt;
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
    error::KafkaResult,
    message::BorrowedMessage,
    Message,
};
use serde_json::Deserializer;
use serde_path_to_error::deserialize;
use tokio_stream::Stream;
use tracing::{debug, error, info};

use crate::error::ConsumerError;
use crate::model::EventStream;

/* --------------------------------------------------------------------- */
/* Context                                                               */

/// Logs rebalance lifecycle & commit callbacks via [`tracing`].
struct TracingContext;
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
            .set("security.protocol", "SASL_SSL")
            .set("sasl.mechanisms", "SCRAM-SHA-512")
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
    pub fn stream(&self) -> impl Stream<Item = KafkaResult<BorrowedMessage<'_>>> + '_ {
        self.inner.stream()
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
