pub mod consumer;
pub mod model;
pub mod producer;
pub mod utils;

mod error;

#[cfg(test)]
mod tests {
    use chrono:: Utc;
    use uuid::Uuid;
    use crate::model::v1::EventStream;

    #[test]
    fn check_message_serialization() {
        let message = EventStream {
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
        let serialized = serde_json::to_string(&message).unwrap();
        let deserialized: EventStream = serde_json::from_str(&serialized).unwrap();
        assert_eq!(message.id, deserialized.id);
        assert_eq!(message.event_source, deserialized.event_source);
    }
}
