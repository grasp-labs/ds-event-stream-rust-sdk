// event_stream.rs
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use uuid::Uuid;

/* —————————————————————————— envelope ———— */

/// Generic Event-Stream envelope (Event V1).
///
/// This struct matches the Event V1 schema exactly.
/// All required and optional fields are included, with correct types.
/// The `body` field is a JSON object (serde_json::Value).
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
    pub body: Option<Value>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub body_uri: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<Value>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Value>,
}
