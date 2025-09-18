# DS Event Stream Rust SDK

[![Crates.io version](https://img.shields.io/crates/v/ds-event-stream-rust-sdk.svg)](https://crates.io/crates/ds-event-stream-rust-sdk)
[![License: MIT OR Apache-2.0](https://img.shields.io/badge/License-MIT%20OR%20Apache--2.0-blue.svg)](https://opensource.org/licenses/MIT)
[![Rust](https://img.shields.io/badge/rust-1.76%2B-blue.svg)](https://www.rust-lang.org)

A Rust SDK for interacting with the DS Event Stream via Kafka. This library provides a clean, async interface for producing and consuming events from the DS Event Stream.

## Features

- **Kafka Producer** - Send events to the DS Event Stream
- **Kafka Consumer** - Consume events from the DS Event Stream
- **Event Models** - Pre-defined event structures for DS Event Stream
- **Async Support** - Built on Tokio for high-performance async operations
- **Error Handling** - Comprehensive error types for robust applications

## Installation

Add this to your `Cargo.toml`:

```toml
[dependencies]
ds-event-stream-rust-sdk = "0.1.0"
```

Or use cargo add:

```sh
cargo add ds-event-stream-rust-sdk
```

## Quick Start

### Producer Example

```rust
use ds_event_stream_rust_sdk::{KafkaProducer, EventStream};
use tracing::info;
use uuid::Uuid;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let producer = KafkaProducer::default("username", "password")?;
    let event = EventStream::new(
        Uuid::new_v4(), // session_id
        Uuid::new_v4(), // tenant_id
        "pipeline-service".to_string(), // event_source
        "pipeline-created".to_string(), // event_type
        "user-42".to_string(), // created_by
        None, // request_id
        None, // owner_id
        None, // product_id
        None, // product_schema_uri
        None, // event_source_uri
        None, // affected_entity_uri
        None, // message
        Some(serde_json::json!({"pipeline_id": "pipeline-123"})), // payload
        None, // payload_uri
        None, // context
        None, // context_uri
        None, // metadata
        None, // tags
    );
    producer.send_message("user-created", "user-42", &event, None).await?;
    info!("Event sent to Kafka");
    Ok(())
}
```

### Consumer Example

```rust
use ds_event_stream_rust_sdk::KafkaConsumer;
use tokio_stream::StreamExt;
use tracing::{error, info};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize consumer
    let consumer = KafkaConsumer::default(&["user-created", "user-updated"], "username", "password")?;
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

## Environment Variables

The SDK uses the following environment variables:

- `KAFKA_BOOTSTRAP_SERVERS` - Kafka broker addresses (required)
- `KAFKA_CONSUMER_GROUP` - Consumer group ID (required for consumers)

## License

This project is licensed under either of

- Apache License, Version 2.0, ([LICENSE-APACHE](LICENSE-APACHE) or <http://www.apache.org/licenses/LICENSE-2.0>)
- MIT License ([LICENSE-MIT](LICENSE-MIT) or <http://opensource.org/licenses/MIT>)

at your option.
