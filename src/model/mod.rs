//! Data models for the Kafka consumer.

pub mod topics;
pub mod v1;

// Re-export the public API
pub use v1::EventStream;
