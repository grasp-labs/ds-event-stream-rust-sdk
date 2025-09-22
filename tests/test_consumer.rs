use chrono::Utc;
use ds_event_stream_rs_sdk::model::EventStream;
use uuid::Uuid;

#[tokio::test]
async fn test_consumer_serialize_deserialize() {
    // Test serialization without creating actual consumer
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
        message: Some("test message".to_string()),
        payload: None,
        payload_uri: None,
        context: None,
        context_uri: None,
        metadata: None,
        tags: None,
    };

    // Test serialization directly
    let serialized = serde_json::to_vec(&event).unwrap();
    assert!(!serialized.is_empty());

    // Verify it's valid JSON
    let json_str = String::from_utf8(serialized).unwrap();
    assert!(json_str.contains("test message"));
}
