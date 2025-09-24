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
//! ds-event-stream-rs-sdk = "0.1.0"
//! ```
//!
//! ### Basic Usage
//!
//! ```no_run
//! use ds_event_stream_rs_sdk::{
//!     model::{EventStream, topics::Topic},
//!     producer::KafkaProducer,
//!     consumer::KafkaConsumer,
//! };
//! use ds_event_stream_rs_sdk::error::{Result, SDKError};
//! use ds_event_stream_rs_sdk::utils::{get_bootstrap_servers, Environment, ClientCredentials};
//!
//! use uuid::Uuid;
//! use chrono::Utc;
//! use tokio_stream::StreamExt;
//! use rdkafka::message::Message;
//!
//! #[tokio::main]
//! async fn main() -> Result<(), SDKError> {
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
//!     let bootstrap_servers = get_bootstrap_servers(Environment::Development, false);
//!     let credentials = ClientCredentials { username: "username".to_string(), password: "password".to_string() };
//!
//!     let producer = KafkaProducer::default(&bootstrap_servers, &credentials)?;
//!     producer.send_event(&Topic::IdpIdentityUserCreated, "user-123", &event, None).await?;
//!
//!     let consumer = KafkaConsumer::default(&bootstrap_servers, &[Topic::IdpIdentityUserCreated], "group-id", &credentials)?;
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
//! ## Error Handling
//!
//! The SDK provides comprehensive error types:
//!
//! - [`error::SDKError`]: Errors related to the SDK
//! - [`producer::error::ProducerError`]: Errors related to message production
//! - [`consumer::error::ConsumerError`]: Errors related to message consumption
//! - [`utils::error::UtilsError`]: Errors related to administrative utilities
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
