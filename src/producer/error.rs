//! Producer error module.
//!
//! This module contains error types specific to message production operations.

use thiserror::Error;

// region: --> ProducerError

/// Errors that can occur during message production operations.
///
/// This enum covers all possible errors when creating producers or sending messages,
/// including Kafka connection issues, JSON serialization problems, and configuration errors.
#[derive(Error, Debug)]
pub enum ProducerError {
    /// Kafka client or operation errors
    #[error("Kafka error: {0}")]
    Kafka(#[from] rdkafka::error::KafkaError),

    /// JSON serialization/deserialization errors
    #[error("JSON error: {0}")]
    Json(#[from] serde_json::Error),

    /// Errors from utility operations (bootstrap servers, etc.)
    #[error("Utils error: {0}")]
    Utils(#[from] crate::utils::error::UtilsError),
}

// endregion: --> ProducerError
