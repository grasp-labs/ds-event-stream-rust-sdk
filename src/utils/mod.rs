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
//! use ds_event_stream_rust_sdk::utils::{list_topics, get_topic};
//!
//! let topic = get_topic("bootstrap_servers", "username", "password", "topic_name");
//! let topics = list_topics("bootstrap_servers", "username", "password");
//! ```
use rdkafka::admin::AdminClient;
use rdkafka::client::DefaultClientContext;
use rdkafka::config::ClientConfig;

use tracing::debug;

// region: --> Utils

/// Get metadata for a specific topic.
///
/// # Arguments
///
/// * `bootstrap_servers` - The bootstrap servers for the DS Event Stream.
/// * `username` - The username for the DS Event Stream.
/// * `password` - The password for the DS Event Stream.
/// * `topic_name` - The name of the topic to get metadata for.
///
/// # Returns
///
/// * `Option<String>` - The topic name if found, None otherwise.
pub fn get_topic(
    bootstrap_servers: &str,
    username: &str,
    password: &str,
    topic_name: &str,
) -> Option<String> {
    debug!("Getting topic metadata for topic: {}", topic_name);
    let admin: AdminClient<DefaultClientContext> = ClientConfig::new()
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
        .expect("Failed to create admin client");

    let metadata = admin
        .inner()
        .fetch_metadata(Some(topic_name), std::time::Duration::from_secs(10))
        .expect("Failed to fetch metadata");

    metadata
        .topics()
        .iter()
        .find(|topic| topic.name() == topic_name)
        .map(|topic| topic.name().to_string())
}

/// Get the topics for the DS Event Stream.
///
/// # Arguments
///
/// * `bootstrap_servers` - The bootstrap servers for the DS Event Stream.
/// * `username` - The username for the DS Event Stream.
/// * `password` - The password for the DS Event Stream.
///
/// # Returns
///
/// * `Vec<String>` - The topic names for the DS Event Stream.
pub fn list_topics(bootstrap_servers: &str, username: &str, password: &str) -> Vec<String> {
    debug!("Getting topics for the DS Event Stream");
    let admin: AdminClient<DefaultClientContext> = ClientConfig::new()
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
        .expect("Failed to create admin client");

    let metadata = admin
        .inner()
        .fetch_metadata(None, std::time::Duration::from_secs(10))
        .expect("Failed to fetch metadata");

    metadata
        .topics()
        .iter()
        .map(|topic| topic.name().to_string())
        .collect()
}

// endregion: --> Utils
