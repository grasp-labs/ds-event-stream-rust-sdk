//! # DS Event Stream Rust SDK
//!
//! A comprehensive Rust SDK for working with the DS Event Stream, providing
//! high-level abstractions for producing and consuming events from Kafka.
//!
//! ## Features
//!
//! - **Event Models**: Type-safe event structures with serialization support
//! - **Kafka Producer**: Async producer for sending events to Kafka topics
//! - **Kafka Consumer**: Stream-based consumer for processing events
//! - **Topic Management**: Predefined topic constants and utilities
//! - **Admin Utilities**: Helper functions for Kafka cluster management
//!
//! ## Quick Start
//!
//! Add to your `Cargo.toml`:
//! ```toml
//! [dependencies]
//! ds-event-stream-rust-sdk = "0.1.0"
//! ```
//!
//! ### Basic Usage
//!
//! ```no_run
//! use ds_event_stream_rust_sdk::{
//!     model::{EventStream, topics::Topic},
//!     producer::KafkaProducer,
//!     consumer::KafkaConsumer,
//! };
//! use uuid::Uuid;
//! use chrono::Utc;
//! use tokio_stream::StreamExt;
//! use rdkafka::message::Message;
//!
//! #[tokio::main]
//! async fn main() -> Result<(), Box<dyn std::error::Error>> {
//!     // Create an event
//!     let event = EventStream::new(
//!         Uuid::new_v4(),
//!         Uuid::new_v4(),
//!         "my-service".to_string(),
//!         "user.created".to_string(),
//!         "system".to_string(),
//!         None, None, None, None, None, None, None, None, None, None, None, None, None
//!     );
//!
//!     // Send event
//!     let producer = KafkaProducer::default("username", "password")?;
//!     producer.send_message(&Topic::IdpIdentityUserCreated, "user-123", &event, None).await?;
//!
//!     // Consume events
//!     let consumer = KafkaConsumer::default(&[Topic::IdpIdentityUserCreated], "group-id", "username", "password")?;
//!     let mut stream = consumer.stream();
//!
//!     while let Some(result) = stream.next().await {
//!         match result {
//!             Ok(msg) => println!("Received: {:?}", msg.topic()),
//!             Err(e) => eprintln!("Error: {}", e),
//!         }
//!     }
//!
//!     Ok(())
//! }
//! ```
//!
//! ## Environment Variables
//!
//! The SDK uses the following environment variables:
//!
//! - `KAFKA_BOOTSTRAP_SERVERS`: Comma-separated list of Kafka brokers
//! - `KAFKA_USERNAME`: Username for SASL authentication
//! - `KAFKA_PASSWORD`: Password for SASL authentication
//!
//! ## Error Handling
//!
//! The SDK provides comprehensive error types:
//!
//! - [`error::ProducerError`]: Errors related to message production
//! - [`error::ConsumerError`]: Errors related to message consumption
//!
//! ## Modules
//!
//! - [`model`]: Event data structures and topic definitions
//! - [`producer`]: Kafka producer for sending events
//! - [`consumer`]: Kafka consumer for receiving events
//! - [`utils`]: Administrative utilities for Kafka management
//! - [`error`]: Error types for the SDK

pub mod consumer;
pub mod error;
pub mod model;
pub mod producer;
pub mod utils;
