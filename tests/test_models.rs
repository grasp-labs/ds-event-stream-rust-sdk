use chrono::Utc;
use ds_event_stream_rust_sdk::model::topics::Topic;
use ds_event_stream_rust_sdk::model::EventStream;
use uuid::Uuid;

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
        payload: None,
        payload_uri: None,
        context: None,
        context_uri: None,
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
    assert_eq!(event.payload, deserialized.payload);
    assert_eq!(event.payload_uri, deserialized.payload_uri);
    assert_eq!(event.metadata, deserialized.metadata);
    assert_eq!(event.tags, deserialized.tags);
}

#[test]
fn test_topic_names() {
    // Pipeline Job
    assert_eq!(
        Topic::DsPipelineJobRequested.name(),
        "ds.pipeline.job.requested.v1"
    );
    assert_eq!(
        Topic::DsPipelineJobStarted.name(),
        "ds.pipeline.job.started.v1"
    );
    assert_eq!(
        Topic::DsPipelineJobCompleted.name(),
        "ds.pipeline.job.completed.v1"
    );
    assert_eq!(
        Topic::DsPipelineJobFailed.name(),
        "ds.pipeline.job.failed.v1"
    );

    // Pipeline Injection Task
    assert_eq!(
        Topic::DsPipelineInjectionTaskRequested.name(),
        "ds.pipeline.injection.task.requested.v1"
    );
    assert_eq!(
        Topic::DsPipelineInjectionTaskStarted.name(),
        "ds.pipeline.injection.task.started.v1"
    );
    assert_eq!(
        Topic::DsPipelineInjectionTaskCompleted.name(),
        "ds.pipeline.injection.task.completed.v1"
    );
    assert_eq!(
        Topic::DsPipelineInjectionTaskFailed.name(),
        "ds.pipeline.injection.task.failed.v1"
    );

    // Pipeline Transformation Task
    assert_eq!(
        Topic::DsPipelineTransformationTaskRequested.name(),
        "ds.pipeline.transformation.task.requested.v1"
    );
    assert_eq!(
        Topic::DsPipelineTransformationTaskStarted.name(),
        "ds.pipeline.transformation.task.started.v1"
    );
    assert_eq!(
        Topic::DsPipelineTransformationTaskCompleted.name(),
        "ds.pipeline.transformation.task.completed.v1"
    );
    assert_eq!(
        Topic::DsPipelineTransformationTaskFailed.name(),
        "ds.pipeline.transformation.task.failed.v1"
    );

    // Pipeline Synchronize Task
    assert_eq!(
        Topic::DsPipelineSynchronizeTaskRequested.name(),
        "ds.pipeline.synchronize.task.requested.v1"
    );
    assert_eq!(
        Topic::DsPipelineSynchronizeTaskStarted.name(),
        "ds.pipeline.synchronize.task.started.v1"
    );
    assert_eq!(
        Topic::DsPipelineSynchronizeTaskCompleted.name(),
        "ds.pipeline.synchronize.task.completed.v1"
    );
    assert_eq!(
        Topic::DsPipelineSynchronizeTaskFailed.name(),
        "ds.pipeline.synchronize.task.failed.v1"
    );

    // Pipeline Synchronize Job
    assert_eq!(
        Topic::DsPipelineSynchronizeJobRequested.name(),
        "ds.pipeline.synchronize.job.requested.v1"
    );
    assert_eq!(
        Topic::DsPipelineSynchronizeJobStarted.name(),
        "ds.pipeline.synchronize.job.started.v1"
    );
    assert_eq!(
        Topic::DsPipelineSynchronizeJobCompleted.name(),
        "ds.pipeline.synchronize.job.completed.v1"
    );
    assert_eq!(
        Topic::DsPipelineSynchronizeJobFailed.name(),
        "ds.pipeline.synchronize.job.failed.v1"
    );

    // Pipeline Clone Job
    assert_eq!(
        Topic::DsPipelineCloneJobRequested.name(),
        "ds.pipeline.clone.job.requested.v1"
    );
    assert_eq!(
        Topic::DsPipelineCloneJobStarted.name(),
        "ds.pipeline.clone.job.started.v1"
    );
    assert_eq!(
        Topic::DsPipelineCloneJobCompleted.name(),
        "ds.pipeline.clone.job.completed.v1"
    );
    assert_eq!(
        Topic::DsPipelineCloneJobFailed.name(),
        "ds.pipeline.clone.job.failed.v1"
    );
}

#[test]
fn test_event_stream_new() {
    let session_id = Uuid::new_v4();
    let tenant_id = Uuid::new_v4();
    let event_source = "test_source".to_string();
    let event_type = "test_type".to_string();
    let created_by = "test_user".to_string();

    let event = EventStream::new(
        session_id,
        tenant_id,
        event_source.clone(),
        event_type.clone(),
        created_by.clone(),
        None,
        None,
        None,
        None,
        None,
        None,
        None,
        None,
        None,
        None,
        None,
        None,
        None,
    );

    assert_eq!(event.session_id, session_id);
    assert_eq!(event.tenant_id, tenant_id);
    assert_eq!(event.event_source, event_source);
    assert_eq!(event.event_type, event_type);
    assert_eq!(event.created_by, created_by);
    assert!(!event.id.is_nil());
    // md5_hash is empty when payload is None
    assert!(event.md5_hash.is_empty());
}
