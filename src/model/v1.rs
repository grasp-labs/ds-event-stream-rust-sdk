// event_stream.rs
use std::hash::{Hash, Hasher};

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use uuid::Uuid;

// region: --> EventStream

/// Generic Event-Stream envelope (Event V1).
///
/// This struct matches the Event V1 schema exactly.
/// All required and optional fields are included, with correct types.
#[derive(Debug, Serialize, Deserialize)]
pub struct EventStream {
    /* -------- required core ------------------------------------------- */
    pub id: Uuid,
    pub session_id: Uuid,
    pub tenant_id: Uuid,
    pub event_source: String,
    pub event_type: String,
    pub timestamp: DateTime<Utc>,
    pub created_by: String,
    pub md5_hash: String,

    /* -------- optional context ---------------------------------------- */
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<Uuid>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub owner_id: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub product_id: Option<Uuid>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub product_schema_uri: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub event_source_uri: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub affected_entity_uri: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub payload: Option<Value>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub payload_uri: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub context: Option<Value>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub context_uri: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<Value>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Value>,
}

impl EventStream {
    /// Compute hash for the given payload
    /// # Arguments
    /// * `payload` - The payload to compute the hash for
    /// # Returns
    /// * `String` - The hash of the payload
    fn compute_payload_hash(payload: &Option<Value>) -> String {
        match payload {
            Some(payload) => {
                let payload_str = serde_json::to_string(payload).unwrap_or_default();
                let mut hasher = std::collections::hash_map::DefaultHasher::new();
                payload_str.hash(&mut hasher);
                format!("{:x}", hasher.finish())
            }
            None => String::new(),
        }
    }

    /// Create a new EventStream
    /// # Arguments
    /// * `session_id` - The session ID
    /// * `tenant_id` - The tenant ID
    /// * `event_source` - The event source
    /// * `event_type` - The event type
    /// * `created_by` - The created by
    /// * `request_id` - The request ID
    /// * `owner_id` - The owner ID
    /// * `product_id` - The product ID
    /// * `product_schema_uri` - The product schema URI
    /// * `event_source_uri` - The event source URI
    /// * `affected_entity_uri` - The affected entity URI
    /// * `message` - The message
    /// * `payload` - The payload
    /// * `payload_uri` - The payload URI
    /// * `context` - The context
    /// * `context_uri` - The context URI
    /// * `metadata` - The metadata
    /// * `tags` - The tags
    /// # Returns
    /// * `EventStream` - The new EventStream
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        session_id: Uuid,
        tenant_id: Uuid,
        event_source: String,
        event_type: String,
        created_by: String,
        request_id: Option<Uuid>,
        owner_id: Option<String>,
        product_id: Option<Uuid>,
        product_schema_uri: Option<String>,
        event_source_uri: Option<String>,
        affected_entity_uri: Option<String>,
        message: Option<String>,
        payload: Option<Value>,
        payload_uri: Option<String>,
        context: Option<Value>,
        context_uri: Option<String>,
        metadata: Option<Value>,
        tags: Option<Value>,
    ) -> Self {
        let md5_hash = Self::compute_payload_hash(&payload);
        Self {
            id: Uuid::new_v4(),
            session_id,
            tenant_id,
            event_source,
            event_type,
            timestamp: Utc::now(),
            created_by,
            md5_hash,
            request_id,
            owner_id,
            product_id,
            product_schema_uri,
            event_source_uri,
            affected_entity_uri,
            message,
            payload,
            payload_uri,
            context,
            context_uri,
            metadata,
            tags,
        }
    }
}

// endregion: --> EventStream
