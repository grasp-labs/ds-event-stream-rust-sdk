# DS Event Stream Rust SDK

[![Crates.io version](https://img.shields.io/crates/v/ds-event-stream-rs-sdk.svg)](https://crates.io/crates/ds-event-stream-rs-sdk)
[![License: MIT OR Apache-2.0](https://img.shields.io/badge/License-MIT%20OR%20Apache--2.0-blue.svg)](https://opensource.org/licenses/MIT)
[![Rust](https://img.shields.io/badge/rust-1.76%2B-blue.svg)](https://www.rust-lang.org)

A Rust SDK for interacting with the DS Event Stream via Kafka. This library provides a clean, async interface for producing and consuming events from the DS Event Stream.

## Features

- **Kafka Producer** - Send events to the DS Event Stream with structured logging
- **Kafka Consumer** - Consume events from the DS Event Stream with stream-based processing
- **Event Models** - Pre-defined event structures for DS Event Stream (EventStream v1)
- **Topic Management** - Predefined topic constants and utilities
- **Admin Utilities** - Helper functions for Kafka cluster management
- **Async Support** - Built on Tokio for high-performance async operations
- **Error Handling** - Comprehensive error types for robust applications
- **Environment Support** - Built-in support for Development and Production environments

## Installation

Add this to your `Cargo.toml`:

```toml
[dependencies]
ds-event-stream-rs-sdk = "0.1.0"
```

Or use cargo add:

```sh
cargo add ds-event-stream-rs-sdk
```

## Quick Start

### Producer Example

```rust
use ds_event_stream_rs_sdk::producer::KafkaProducer;
use ds_event_stream_rs_sdk::model::v1::EventStream;
use ds_event_stream_rs_sdk::model::topics::Topic;
use ds_event_stream_rs_sdk::utils::{get_bootstrap_servers, Environment, ClientCredentials};
use tracing::info;
use uuid::Uuid;
use chrono::Utc;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let bootstrap_servers = get_bootstrap_servers(Environment::Development, false);
    let credentials = ClientCredentials {
        username: "username".to_string(),
        password: "password".to_string()
    };
    let producer = KafkaProducer::default(&bootstrap_servers, &credentials)?;

    let event = EventStream {
        id: Uuid::new_v4(),
        session_id: Uuid::new_v4(),
        tenant_id: Uuid::new_v4(),
        event_source: "pipeline-service".to_string(),
        event_type: "pipeline-created".to_string(),
        timestamp: Utc::now(),
        created_by: "user-42".to_string(),
        md5_hash: "hash".to_string(),
        request_id: None,
        owner_id: None,
        product_id: None,
        product_schema_uri: None,
        event_source_uri: None,
        affected_entity_uri: None,
        message: None,
        payload: Some(serde_json::json!({"pipeline_id": "pipeline-123"})),
        payload_uri: None,
        context: None,
        context_uri: None,
        metadata: None,
        tags: None,
    };

    producer.send_event(&Topic::DsPipelineJobRequested, "user-42", &event, None).await?;
    info!("Event sent to Kafka");
    Ok(())
}
```

### Consumer Example

```rust
use ds_event_stream_rs_sdk::consumer::KafkaConsumer;
use ds_event_stream_rs_sdk::model::topics::Topic;
use ds_event_stream_rs_sdk::utils::{get_bootstrap_servers, Environment, ClientCredentials};
use tokio_stream::StreamExt;
use rdkafka::message::Message;
use tracing::{error, info};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let bootstrap_servers = get_bootstrap_servers(Environment::Development, false);
    let credentials = ClientCredentials {
        username: "username".to_string(),
        password: "password".to_string()
    };
    // Initialize consumer
    let consumer = KafkaConsumer::default(
        &bootstrap_servers,
        &[Topic::DsPipelineJobRequested],
        "group-id",
        &credentials
    )?;
    let mut stream = consumer.stream();

    // Process events
    while let Some(result) = stream.next().await {
        match result {
            Ok(msg) => {
                info!("Received message on topic: {}", msg.topic());
                if let Some(payload) = msg.payload() {
                    info!("Payload: {:?}", std::str::from_utf8(payload)?);
                }
            }
            Err(e) => error!("Kafka error: {}", e),
        }
    }

    Ok(())
}
```

## Configuration

The SDK provides utility functions for getting bootstrap servers for different environments:

- `get_bootstrap_servers(Environment::Development, false)` - Development external servers
- `get_bootstrap_servers(Environment::Development, true)` - Development internal servers
- `get_bootstrap_servers(Environment::Production, false)` - Production external servers
- `get_bootstrap_servers(Environment::Production, true)` - Production internal servers

Authentication is handled via `ClientCredentials` struct with username and password fields.

## License

This project is licensed under either of

- Apache License, Version 2.0, ([LICENSE-APACHE](LICENSE-APACHE) or <http://www.apache.org/licenses/LICENSE-2.0>)
- MIT License ([LICENSE-MIT](LICENSE-MIT) or <http://opensource.org/licenses/MIT>)

at your option.
