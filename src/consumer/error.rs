//! Consumer error module.
//!
//! This module contains error types specific to message consumption operations.

use std::str::Utf8Error;
use thiserror::Error;

// region: --> ConsumerError

/// Errors that can occur during message consumption operations.
///
/// This enum covers all possible errors when creating consumers or processing messages,
/// including Kafka connection issues, message parsing problems, and validation errors.
#[derive(Error, Debug)]
pub enum ConsumerError {
    /// Kafka client or operation errors
    #[error("Kafka error: {0}")]
    Kafka(#[from] rdkafka::error::KafkaError),

    /// JSON serialization/deserialization errors
    #[error("JSON error: {0}")]
    Json(#[from] serde_json::Error),

    /// UTF-8 string conversion errors
    #[error("UTF-8 error: {0}")]
    Utf8(#[from] Utf8Error),

    /// Invalid message payload
    #[error("Invalid payload: {0}")]
    InvalidPayload(String),

    /// Message validation errors
    #[error("Validation error: {0}")]
    Validation(String),

    /// Errors from utility operations (bootstrap servers, etc.)
    #[error("Utils error: {0}")]
    Utils(#[from] crate::utils::error::UtilsError),
}

// endregion: --> ConsumerError
