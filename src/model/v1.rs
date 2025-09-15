// event_stream.rs
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use uuid::Uuid;

// region: --> EventStream

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

// endregion: --> EventStream

// region: --> Tests

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_event_stream_serialization() {
        let event = EventStream {
            id: Uuid::new_v4(),
            session_id: Uuid::new_v4(),
            tenant_id: Uuid::new_v4(),
            event_source: "test".to_string(),
            event_type: "test".to_string(),
            timestamp: Utc::now(),
            created_by: "test".to_string(),
            md5_hash: "test".to_string(),
            request_id: None,
            owner_id: None,
            product_id: None,
            product_schema_uri: None,
            event_source_uri: None,
            affected_entity_uri: None,
            message: None,
            body: None,
            body_uri: None,
            metadata: None,
            tags: None,
        };
        let serialized = serde_json::to_string(&event).unwrap();
        assert!(!serialized.is_empty());
        let deserialized: EventStream = serde_json::from_str(&serialized).unwrap();
        assert_eq!(event.id, deserialized.id);
        assert_eq!(event.event_source, deserialized.event_source);
        assert_eq!(event.event_type, deserialized.event_type);
        assert_eq!(event.timestamp, deserialized.timestamp);
        assert_eq!(event.created_by, deserialized.created_by);
        assert_eq!(event.md5_hash, deserialized.md5_hash);
        assert_eq!(event.request_id, deserialized.request_id);
        assert_eq!(event.owner_id, deserialized.owner_id);
        assert_eq!(event.product_id, deserialized.product_id);
        assert_eq!(event.product_schema_uri, deserialized.product_schema_uri);
        assert_eq!(event.event_source_uri, deserialized.event_source_uri);
        assert_eq!(event.affected_entity_uri, deserialized.affected_entity_uri);
        assert_eq!(event.message, deserialized.message);
        assert_eq!(event.body, deserialized.body);
        assert_eq!(event.body_uri, deserialized.body_uri);
        assert_eq!(event.metadata, deserialized.metadata);
        assert_eq!(event.tags, deserialized.tags);
    }
}

// endregion: --> Tests
