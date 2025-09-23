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
//! use ds_event_stream_rs_sdk::consumer::KafkaConsumer;
//! use ds_event_stream_rs_sdk::model::topics::Topic;
//! use ds_event_stream_rs_sdk::model::v1::EventStream;
//! use ds_event_stream_rs_sdk::error::{Result, SDKError};
//! use ds_event_stream_rs_sdk::utils::{get_bootstrap_servers, Environment, ClientCredentials   };
//!
//! use tokio_stream::StreamExt;
//! use tracing::{info, error};
//! use rdkafka::message::Message;
//!
//! #[tokio::main]
//! async fn main() -> Result<(), SDKError> {
//!     let bootstrap_servers = get_bootstrap_servers(Environment::Development, false);
//!     let credentials = ClientCredentials { username: "username".to_string(), password: "password".to_string() };
//!
//!     let consumer = KafkaConsumer::default(&bootstrap_servers, &[Topic::DsPipelineJobRequested], "group-id", &credentials)?;
//!     let mut stream = consumer.stream();
//!
//!     while let Some(result) = stream.next().await {
//!         match result {
//!             Ok(msg) => {
//!                 info!("Received message on topic: {}", msg.topic());
//!                 match consumer.deserialize_message(&msg) {
//!                     Ok(event) => info!("Deserialized event: {:?}", event),
//!                     Err(e) => error!("Failed to deserialize message: {}", e),
//!                 }
//!             }
//!             Err(e) => error!("Kafka error: {}", e),
//!         }
//!     }
//!     Ok(())
//! }
//! ```
pub mod error;

use rdkafka::{
    client::ClientContext,
    config::{ClientConfig, RDKafkaLogLevel},
    consumer::{stream_consumer::StreamConsumer, BaseConsumer, CommitMode, Consumer, ConsumerContext, Rebalance},
    message::BorrowedMessage,
    Message,
};
use serde_json::Deserializer;
use serde_path_to_error::deserialize;
use tracing::{debug, error, info};

use crate::error::{Result, SDKError};
use crate::model::topics::Topic;
use crate::model::v1::EventStream;
use crate::utils::ClientCredentials;

use error::ConsumerError;

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
/// use ds_event_stream_rs_sdk::consumer::TracingContext;
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
        debug!(?r, "pre‑rebalance");
    }
    fn post_rebalance<'a>(&self, _c: &BaseConsumer<TracingContext>, r: &Rebalance<'a>) {
        debug!(?r, "post‑rebalance");
    }
    fn commit_callback(&self, res: rdkafka::error::KafkaResult<()>, offs: &rdkafka::TopicPartitionList) {
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
    /// * `config` - The consumer configuration
    ///
    /// # Returns
    ///
    /// * `Result<Self, SDKError>` - The result of the operation
    ///
    /// # Errors
    ///
    /// * [`SDKError::Consumer`] - If the consumer fails to create.
    ///
    pub fn new(topics: &[Topic], config: ClientConfig) -> Result<Self> {
        let inner: StreamConsumer<_> = config
            .create_with_context(TracingContext)
            .map_err(ConsumerError::Kafka)?;

        let topic_refs: Vec<&str> = topics.iter().map(|t| t.as_ref()).collect();
        inner.subscribe(&topic_refs).map_err(ConsumerError::Kafka)?;

        info!(topics = ?topic_refs, "Kafka consumer initialised");
        Ok(Self { inner })
    }

    /// Default configuration.
    ///
    /// # Arguments
    ///
    /// * `topics` - The topics to subscribe to
    /// * `group_id` - The group id to use for the consumer
    /// * `credentials` - The credentials to use for authentication
    ///
    /// # Returns
    ///
    /// * `Result<Self, SDKError>` - The result of the operation
    ///
    /// # Errors
    ///
    /// * [`SDKError::Consumer`] - If the consumer fails to create.
    ///
    pub fn default(
        bootstrap_servers: &str,
        topics: &[Topic],
        group_id: &str,
        credentials: &ClientCredentials,
    ) -> Result<Self> {
        let inner: StreamConsumer<_> = ClientConfig::new()
            .set("group.id", group_id)
            .set("bootstrap.servers", bootstrap_servers)
            .set("session.timeout.ms", "6000")
            .set("enable.partition.eof", "false")
            .set("heartbeat.interval.ms", "3000")
            .set("max.poll.interval.ms", "300000")
            .set("auto.offset.reset", "earliest")
            .set("enable.auto.commit", "false")
            .set("fetch.min.bytes", "1")
            .set("max.partition.fetch.bytes", "1048576")
            .set("security.protocol", "SASL_PLAINTEXT")
            .set("sasl.mechanism", "SCRAM-SHA-512")
            .set("sasl.username", credentials.username.clone())
            .set("sasl.password", credentials.password.clone())
            .set_log_level(RDKafkaLogLevel::Info)
            .create_with_context(TracingContext)
            .map_err(ConsumerError::Kafka)?;

        let topic_refs: Vec<&str> = topics.iter().map(|t| t.as_ref()).collect();
        inner.subscribe(&topic_refs).map_err(ConsumerError::Kafka)?;

        info!(topics = ?topic_refs, "Kafka consumer initialised");
        Ok(Self { inner })
    }

    /* ---- helpers ----------------------------------------------------- */

    /// Async stream of messages (`KafkaResult<BorrowedMessage>`).
    ///
    /// # Returns
    ///
    /// * `impl Stream<Item = KafkaResult<BorrowedMessage<'_>>> + '_` - The stream of messages
    pub fn stream(&self) -> impl tokio_stream::Stream<Item = rdkafka::error::KafkaResult<BorrowedMessage<'_>>> + '_ {
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
    /// # Arguments
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
    /// # Arguments
    ///
    /// * `msg` - The message to deserialize
    ///
    /// # Returns
    ///
    /// * `Result<EventStream, SDKError>` - The result of the operation
    ///
    /// # Errors
    ///
    /// * [`SDKError::Consumer`] - If the consumer fails to deserialize the message.
    ///
    pub fn deserialize_message(&self, msg: &BorrowedMessage<'_>) -> Result<EventStream> {
        let payload = msg
            .payload()
            .ok_or_else(|| ConsumerError::InvalidPayload("Empty message payload".to_string()))?;
        let payload_str = std::str::from_utf8(payload).map_err(ConsumerError::Utf8)?;

        let de = &mut Deserializer::from_str(payload_str);
        match deserialize::<_, EventStream>(de) {
            Ok(event) => Ok(event),
            Err(e) => Err(SDKError::Consumer(ConsumerError::InvalidPayload(format!(
                "Deserialization error at {}: {}",
                e.path(),
                e
            )))),
        }
    }

    /// Serializes an `EventStream<T>` into a Kafka message.
    ///
    /// * `T` must implement [`serde::Serialize`].
    ///
    /// # Arguments
    ///
    /// * `msg` - The message to serialize
    ///
    /// # Returns
    ///
    /// * `Result<Vec<u8>, SDKError>` - The result of the operation
    ///
    /// # Errors
    ///
    /// * [`SDKError::Consumer`] - If the consumer fails to serialize the message.
    ///
    pub fn serialize_message(&self, msg: &EventStream) -> Result<Vec<u8>> {
        Ok(serde_json::to_vec(msg).map_err(ConsumerError::Json)?)
    }
}
