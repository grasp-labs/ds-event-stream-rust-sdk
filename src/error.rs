//! Error module.
//!
//! ## Overview
//! This module contains the unified error types for the DS Event Stream Rust SDK.
//! All SDK functions return `Result<T, SDKError>` for consistent error handling.
//!
//! ## Error Hierarchy
//!
//! - [`SDKError`]: Top-level error enum that unifies all module errors
//! - [`ProducerError`]: Errors from message production operations
//! - [`ConsumerError`]: Errors from message consumption operations
//! - [`UtilsError`]: Errors from administrative utilities
//!

use thiserror::Error;

use crate::consumer::error::ConsumerError;
use crate::producer::error::ProducerError;
use crate::utils::error::UtilsError;

/// Unified result type for all SDK operations.
///
/// This is equivalent to `Result<T, SDKError>` and provides consistent
/// error handling across all SDK modules.
pub type Result<T, E = SDKError> = std::result::Result<T, E>;

// region: --> SDKError

/// Top-level error enum that unifies all SDK module errors.
///
/// This enum provides a single error type for all SDK operations,
/// making error handling consistent and ergonomic for users.
#[derive(Error, Debug)]
pub enum SDKError {
    /// Errors from producer operations (sending messages)
    #[error("Producer error: {0}")]
    Producer(#[from] ProducerError),

    /// Errors from consumer operations (receiving messages)
    #[error("Consumer error: {0}")]
    Consumer(#[from] ConsumerError),

    /// Errors from utility operations (admin functions)
    #[error("Utils error: {0}")]
    Utils(#[from] UtilsError),
}

// endregion: --> SDKError
