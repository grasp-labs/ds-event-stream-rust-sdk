//! Utils error module.
//!
//! This module contains error types specific to administrative utility operations.

use thiserror::Error;

// region: --> UtilsError

/// Errors that can occur during administrative utility operations.
///
/// This enum covers all possible errors when performing admin operations like
/// listing topics, fetching metadata, or managing Kafka cluster resources.
#[derive(Error, Debug)]
pub enum UtilsError {
    /// Kafka client or operation errors
    #[error("Kafka error: {0}")]
    Kafka(#[from] rdkafka::error::KafkaError),

    /// Requested topic was not found
    #[error("Topic not found: {topic_name} for clientId: {client_id}")]
    TopicNotFound { topic_name: String, client_id: String },

    /// No topics found for the client
    #[error("No topics found for clientId: {client_id}")]
    TopicsNotFound { client_id: String },
}

// endregion: --> UtilsError
