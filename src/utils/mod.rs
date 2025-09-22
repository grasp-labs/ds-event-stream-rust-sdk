//! Utils module.
//!
//! ## Overview
//! This module contains utility functions for the DS Event Stream.
//!
//! ## Features
//! * Get the topics for the DS Event Stream.
//!
//! ### Example
//! ```no_run
//! use ds_event_stream_rs_sdk::utils::{list_topics, get_topic};
//!
//! let topic = get_topic("username", "password", "topic_name");
//! let topics = list_topics("username", "password");
//! ```
use std::env;

use rdkafka::admin::AdminClient;
use rdkafka::client::DefaultClientContext;
use rdkafka::config::ClientConfig;
use tracing::debug;

use crate::error::AdminError;

// region: --> helpers

/// Create an admin client.
///
/// # Arguments
///
/// * `username` - The username for the DS Event Stream.
/// * `password` - The password for the DS Event Stream.
///
/// # Returns
///
/// * `AdminClient<DefaultClientContext>` - The admin client.
///
/// # Errors
///
/// * [`AdminError::Kafka`] - If the Kafka client fails to create.
/// * [`AdminError::MissingEnvVar`] - If the environment variable is not set.
///
fn create_admin_client(username: &str, password: &str) -> Result<AdminClient<DefaultClientContext>, AdminError> {
    let bootstrap_servers = env::var("KAFKA_BOOTSTRAP_SERVERS").map_err(|_| AdminError::MissingEnvVar {
        var_name: "KAFKA_BOOTSTRAP_SERVERS".to_string(),
    })?;
    ClientConfig::new()
        .set("bootstrap.servers", bootstrap_servers)
        .set("session.timeout.ms", "6000")
        .set("request.timeout.ms", "30000")
        .set("connections.max.idle.ms", "540000")
        .set("metadata.max.age.ms", "300000")
        .set("security.protocol", "SASL_PLAINTEXT")
        .set("sasl.mechanism", "SCRAM-SHA-512")
        .set("sasl.username", username)
        .set("sasl.password", password)
        .create()
        .map_err(AdminError::Kafka)
}

// endregion: --> helpers

// region: --> Utils

/// Get the name of a specific topic.
///
/// # Arguments
///
/// * `username` - The username for the DS Event Stream.
/// * `password` - The password for the DS Event Stream.
/// * `topic_name` - The name of the topic to get metadata for.
///
/// # Returns
///
/// * `String` - The topic name if found.
///
/// # Errors
///
/// * [`AdminError::Kafka`] - If the Kafka client fails to fetch metadata.
/// * [`AdminError::MissingEnvVar`] - If the environment variable is not set.
/// * [`AdminError::TopicNotFound`] - If the topic is not found.
///
pub fn get_topic(username: &str, password: &str, topic_name: &str) -> Result<String, AdminError> {
    debug!("Getting topic metadata for topic: {}", topic_name);
    let admin: AdminClient<DefaultClientContext> = create_admin_client(username, password)?;
    let metadata = admin
        .inner()
        .fetch_metadata(Some(topic_name), std::time::Duration::from_secs(10))
        .map_err(AdminError::Kafka)?;

    let result = metadata
        .topics()
        .iter()
        .find(|topic| topic.name() == topic_name)
        .map(|topic| topic.name().to_string());

    match result {
        Some(topic_name) => Ok(topic_name),
        None => Err(AdminError::TopicNotFound {
            topic_name: topic_name.to_string(),
            client_id: username.to_string(),
        }),
    }
}

/// List the topics for the DS Event Stream.
///
/// # Arguments
///
/// * `username` - The username for the DS Event Stream.
/// * `password` - The password for the DS Event Stream.
///
/// # Returns
///
/// * `Vec<String>` - The topic names for the DS Event Stream.
///
/// # Errors
///
/// * [`AdminError::Kafka`] - If the Kafka client fails to fetch metadata.
/// * [`AdminError::MissingEnvVar`] - If the environment variable is not set.
///
pub fn list_topics(username: &str, password: &str) -> Result<Vec<String>, AdminError> {
    debug!("Getting topics for the DS Event Stream");
    let admin: AdminClient<DefaultClientContext> = create_admin_client(username, password)?;
    let metadata = admin
        .inner()
        .fetch_metadata(None, std::time::Duration::from_secs(10))
        .map_err(AdminError::Kafka)?;

    let topics: Vec<String> = metadata.topics().iter().map(|topic| topic.name().to_string()).collect();
    if topics.is_empty() {
        return Err(AdminError::TopicsNotFound {
            client_id: username.to_string(),
        });
    }

    Ok(topics)
}

// endregion: --> Utils
