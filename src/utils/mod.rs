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
//! use ds_event_stream_rs_sdk::utils::{list_topics, get_topic, get_bootstrap_servers, Environment, ClientCredentials};
//!
//! let bootstrap_servers = get_bootstrap_servers(Environment::Development, false);
//! let credentials = ClientCredentials { username: "username".to_string(), password: "password".to_string() };
//!
//! let topic = get_topic(&bootstrap_servers, &credentials, "topic_name");
//! let topics = list_topics(&bootstrap_servers, &credentials);
//! ```
pub mod error;

use rdkafka::admin::AdminClient;
use rdkafka::client::DefaultClientContext;
use rdkafka::config::ClientConfig;
use tracing::debug;

use crate::error::{Result, SDKError};

use error::UtilsError;

// region: --> helpers

// Bootstrap server constants
const DEV_INTERNAL_BOOTSTRAP: &str = "kafka-broker-0.kafka-broker-headless.kafka-dev.svc.cluster.local:9092";
const DEV_EXTERNAL_BOOTSTRAP: &str = "b0.dev.kafka.ds.local:9095";
const PROD_INTERNAL_BOOTSTRAP: &str = "kafka-broker-0.kafka-broker-headless.kafka.svc.cluster.local:9092,kafka-broker-1.kafka-broker-headless.kafka.svc.cluster.local:9092,kafka-broker-2.kafka-broker-headless.kafka.svc.cluster.local:9092";
const PROD_EXTERNAL_BOOTSTRAP: &str = "b0.kafka.ds.local:9095,b1.kafka.ds.local:9095,b2.kafka.ds.local:9095";

/// Client credentials for Kafka authentication.
///
/// # Arguments
///
/// * `username` - The username to use for authentication
/// * `password` - The password to use for authentication
///
#[derive(Debug, Clone)]
pub struct ClientCredentials {
    pub username: String,
    pub password: String,
}

/// The environment of the DS Event Stream.
///
/// # Arguments
///
/// * `env` - The environment to get the environment for.
///
/// # Returns
///
/// * `Environment` - The environment of the DS Event Stream.
///
#[derive(Debug, Clone)]
pub enum Environment {
    Development,
    Production,
}

/// Get the bootstrap servers for the DS Event Stream.
///
/// # Arguments
///
/// * `env` - The environment to get the bootstrap servers for.
/// * `use_internal_hostnames` - Whether to use the internal hostnames for the bootstrap servers.
///
/// # Returns
///
/// * `String` - The bootstrap servers for the DS Event Stream.
///
pub fn get_bootstrap_servers(env: Environment, use_internal_hostnames: bool) -> String {
    match env {
        Environment::Development => {
            if use_internal_hostnames {
                DEV_INTERNAL_BOOTSTRAP.to_string()
            } else {
                DEV_EXTERNAL_BOOTSTRAP.to_string()
            }
        }
        Environment::Production => {
            if use_internal_hostnames {
                PROD_INTERNAL_BOOTSTRAP.to_string()
            } else {
                PROD_EXTERNAL_BOOTSTRAP.to_string()
            }
        }
    }
}

/// Create an admin client.
///
/// # Arguments
///
/// * `bootstrap_servers` - The bootstrap servers to create the admin client for.
/// * `username` - The username for the DS Event Stream.
/// * `password` - The password for the DS Event Stream.
///
/// # Returns
///
/// * `AdminClient<DefaultClientContext>` - The admin client.
///
/// # Errors
///
/// * [`UtilsError::Kafka`] - If the Kafka client fails to create.
///
fn create_admin_client(
    bootstrap_servers: &str,
    credentials: &ClientCredentials,
) -> Result<AdminClient<DefaultClientContext>> {
    ClientConfig::new()
        .set("bootstrap.servers", bootstrap_servers)
        .set("session.timeout.ms", "6000")
        .set("request.timeout.ms", "30000")
        .set("connections.max.idle.ms", "540000")
        .set("metadata.max.age.ms", "300000")
        .set("security.protocol", "SASL_PLAINTEXT")
        .set("sasl.mechanism", "SCRAM-SHA-512")
        .set("sasl.username", credentials.username.clone())
        .set("sasl.password", credentials.password.clone())
        .create()
        .map_err(|e| SDKError::from(UtilsError::Kafka(e)))
}

// endregion: --> helpers

// region: --> Utils

/// Get the name of a specific topic.
///
/// # Arguments
///
/// * `bootstrap_servers` - The bootstrap servers to get the topic metadata for.
/// * `credentials` - The credentials for the DS Event Stream.
/// * `topic_name` - The name of the topic to get metadata for.
///
/// # Returns
///
/// * `String` - The topic name if found.
///
/// # Errors
///
/// * [`UtilsError::Kafka`] - If the Kafka client fails to fetch metadata.
/// * [`UtilsError::TopicNotFound`] - If the topic is not found.
///
pub fn get_topic(bootstrap_servers: &str, credentials: &ClientCredentials, topic_name: &str) -> Result<String> {
    debug!("Getting topic metadata for topic: {}", topic_name);
    let admin: AdminClient<DefaultClientContext> = create_admin_client(bootstrap_servers, credentials)?;
    let metadata = admin
        .inner()
        .fetch_metadata(Some(topic_name), std::time::Duration::from_secs(10))
        .map_err(UtilsError::Kafka)?;

    let result = metadata
        .topics()
        .iter()
        .find(|topic| topic.name() == topic_name)
        .map(|topic| topic.name().to_string());

    match result {
        Some(topic_name) => Ok(topic_name),
        None => Err(SDKError::from(UtilsError::TopicNotFound {
            topic_name: topic_name.to_string(),
            client_id: credentials.username.to_string(),
        })),
    }
}

/// List the topics for the DS Event Stream.
///
/// # Arguments
///
/// * `bootstrap_servers` - The bootstrap servers to list the topics for.
/// * `credentials` - The credentials for the DS Event Stream.
///
/// # Returns
///
/// * `Vec<String>` - The topic names for the DS Event Stream.
///
/// # Errors
///
/// * [`UtilsError::Kafka`] - If the Kafka client fails to fetch metadata.
/// * [`UtilsError::TopicsNotFound`] - If the topics are not found.
///
pub fn list_topics(bootstrap_servers: &str, credentials: &ClientCredentials) -> Result<Vec<String>> {
    debug!("Getting topics for the DS Event Stream");
    let admin: AdminClient<DefaultClientContext> = create_admin_client(bootstrap_servers, credentials)?;
    let metadata = admin
        .inner()
        .fetch_metadata(None, std::time::Duration::from_secs(10))
        .map_err(UtilsError::Kafka)?;

    let topics: Vec<String> = metadata.topics().iter().map(|topic| topic.name().to_string()).collect();
    if topics.is_empty() {
        return Err(SDKError::from(UtilsError::TopicsNotFound {
            client_id: credentials.username.to_string(),
        }));
    }

    Ok(topics)
}

// endregion: --> Utils
