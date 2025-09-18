use std::str::Utf8Error;
use thiserror::Error;

// region: --> AdminError

#[derive(Error, Debug)]
pub enum AdminError {
    #[error("Missing environment variable: {var_name}")]
    MissingEnvVar { var_name: String },

    #[error("Kafka error: {0}")]
    Kafka(#[from] rdkafka::error::KafkaError),

    #[error("Topic not found: {topic_name} for clientId: {client_id}")]
    TopicNotFound { topic_name: String, client_id: String },

    #[error("No topics found for clientId: {client_id}")]
    TopicsNotFound { client_id: String },
}

// endregion: --> AdminError

// region: --> ConsumerError

#[derive(Error, Debug)]
pub enum ConsumerError {
    #[error("Kafka error: {0}")]
    Kafka(#[from] rdkafka::error::KafkaError),

    #[error("Missing environment variable: {var_name}")]
    MissingEnvVar { var_name: String },

    #[error("Invalid payload: {0}")]
    InvalidPayload(String),

    #[error("Validation error: {0}")]
    Validation(String),

    #[error("UTF-8 error: {0}")]
    Utf8(#[from] Utf8Error),

    #[error("JSON error: {0}")]
    Json(#[from] serde_json::Error),
}

// endregion: --> ConsumerError

// region: --> ProducerError

#[derive(Error, Debug)]
pub enum ProducerError {
    #[error("Kafka error: {0}")]
    Kafka(#[from] rdkafka::error::KafkaError),

    #[error("Missing environment variable: {var_name}")]
    MissingEnvVar { var_name: String },

    #[error("JSON error: {0}")]
    Json(#[from] serde_json::Error),
}

// endregion: --> ProducerError
