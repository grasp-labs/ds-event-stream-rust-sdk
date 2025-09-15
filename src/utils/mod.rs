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
//! use ds_event_stream_rust_sdk::utils::{list_topics, get_topic, list_acls_cli, list_principal_acls_cli};
//!
//! let topic = get_topic("bootstrap_servers", "username", "password", "topic_name");
//! let topics = list_topics("bootstrap_servers", "username", "password");
//! let acls = list_acls_cli("bootstrap_servers", "username", "password").unwrap();
//! let user_acls = list_principal_acls_cli("bootstrap_servers", "username", "password", "User:alice").unwrap();
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

/// List all ACLs using Kafka CLI tools.
///
/// # Arguments
///
/// * `bootstrap_servers` - The bootstrap servers for the DS Event Stream.
/// * `username` - The username for the DS Event Stream.
/// * `password` - The password for the DS Event Stream.
///
/// # Returns
///
/// * `Result<Vec<String>, Box<dyn std::error::Error>>` - The ACL entries as strings, or an error.
pub fn list_acls_cli(
    bootstrap_servers: &str,
    username: &str,
    password: &str,
) -> Result<Vec<String>, Box<dyn std::error::Error>> {
    debug!("Getting ACLs using Kafka CLI tools");

    // Create a temporary config file for SASL authentication
    let config_content = format!(
        "security.protocol=SASL_PLAINTEXT\n\
         sasl.mechanism=PLAIN\n\
         sasl.jaas.config=org.apache.kafka.common.security.plain.PlainLoginModule required username=\"{}\" password=\"{}\";",
        username, password
    );

    let mut child = std::process::Command::new("kafka-acls.sh")
        .arg("--bootstrap-server")
        .arg(bootstrap_servers)
        .arg("--list")
        .arg("--command-config")
        .arg("/dev/stdin")
        .stdin(std::process::Stdio::piped())
        .stdout(std::process::Stdio::piped())
        .stderr(std::process::Stdio::piped())
        .spawn()?;

    if let Some(mut stdin) = child.stdin.take() {
        use std::io::Write;
        stdin.write_all(config_content.as_bytes())?;
    }

    let output = child.wait_with_output()?;

    if !output.status.success() {
        return Err(format!(
            "Failed to list ACLs: {}",
            String::from_utf8_lossy(&output.stderr)
        )
        .into());
    }

    let acls = String::from_utf8(output.stdout)?
        .lines()
        .filter(|line| !line.is_empty())
        .map(|line| line.to_string())
        .collect();

    Ok(acls)
}

/// List ACLs for a specific principal using Kafka CLI tools.
///
/// # Arguments
///
/// * `bootstrap_servers` - The bootstrap servers for the DS Event Stream.
/// * `username` - The username for the DS Event Stream.
/// * `password` - The password for the DS Event Stream.
/// * `principal` - The principal (user) to get ACLs for.
///
/// # Returns
///
/// * `Result<Vec<String>, Box<dyn std::error::Error>>` - The ACL entries for the principal, or an error.
pub fn list_principal_acls_cli(
    bootstrap_servers: &str,
    username: &str,
    password: &str,
    principal: &str,
) -> Result<Vec<String>, Box<dyn std::error::Error>> {
    debug!(
        "Getting ACLs for principal: {} using Kafka CLI tools",
        principal
    );

    // Create a temporary config file for SASL authentication
    let config_content = format!(
        "security.protocol=SASL_PLAINTEXT\n\
         sasl.mechanism=PLAIN\n\
         sasl.jaas.config=org.apache.kafka.common.security.plain.PlainLoginModule required username=\"{}\" password=\"{}\";",
        username, password
    );

    let mut child = std::process::Command::new("kafka-acls.sh")
        .arg("--bootstrap-server")
        .arg(bootstrap_servers)
        .arg("--list")
        .arg("--principal")
        .arg(principal)
        .arg("--command-config")
        .arg("/dev/stdin")
        .stdin(std::process::Stdio::piped())
        .stdout(std::process::Stdio::piped())
        .stderr(std::process::Stdio::piped())
        .spawn()?;

    if let Some(mut stdin) = child.stdin.take() {
        use std::io::Write;
        stdin.write_all(config_content.as_bytes())?;
    }

    let output = child.wait_with_output()?;

    if !output.status.success() {
        return Err(format!(
            "Failed to list ACLs for principal {}: {}",
            principal,
            String::from_utf8_lossy(&output.stderr)
        )
        .into());
    }

    let acls = String::from_utf8(output.stdout)?
        .lines()
        .filter(|line| !line.is_empty())
        .map(|line| line.to_string())
        .collect();

    Ok(acls)
}

// endregion: --> Utils

// region: --> Tests

#[cfg(test)]
mod tests {
    // Disabled tests that require actual Kafka broker connection
    // #[test]
    // fn test_get_topic() {
    //     let _topic = get_topic("bootstrap_servers", "username", "password", "test_topic");
    // }
    //
    // #[test]
    // fn test_list_topics() {
    //     let _topics = list_topics("bootstrap_servers", "username", "password");
    // }
}

// endregion: --> Tests
