use chrono::Utc;
use ds_event_stream_rs_sdk::model::topics::Topic;
use ds_event_stream_rs_sdk::model::EventStream;
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
    // IDP Identity User Events
    assert_eq!(Topic::IdpIdentityUserCreated.name(), "idp.identity..user.created.v1");
    assert_eq!(Topic::IdpIdentityUserUpdated.name(), "idp.identity..user.updated.v1");
    assert_eq!(Topic::IdpIdentityUserDeleted.name(), "idp.identity..user.deleted.v1");
    assert_eq!(Topic::IdpIdentityUserAuthenticated.name(), "idp.identity..user.authenticated.v1");

    // IDP Identity Tenant Events
    assert_eq!(Topic::IdpIdentityTenantCreated.name(), "idp.identity..tenant.created.v1");
    assert_eq!(Topic::IdpIdentityTenantUpdated.name(), "idp.identity..tenant.updated.v1");
    assert_eq!(Topic::IdpIdentityTenantDeleted.name(), "idp.identity..tenant.deleted.v1");

    // DS Pipeline Job Events
    assert_eq!(Topic::DsPipelineJobRequested.name(), "ds.pipeline..job.requested.v1");
    assert_eq!(Topic::DsPipelineJobStarted.name(), "ds.pipeline..job.started.v1");
    assert_eq!(Topic::DsPipelineJobCompleted.name(), "ds.pipeline..job.completed.v1");
    assert_eq!(Topic::DsPipelineJobFailed.name(), "ds.pipeline..job.failed.v1");

    // DS Pipeline Injection Task Events
    assert_eq!(Topic::DsPipelineInjectionTaskCompleted.name(), "ds.pipeline.injection.task.completed.v1");
    assert_eq!(Topic::DsPipelineInjectionTaskFailed.name(), "ds.pipeline.injection.task.failed.v1");
    assert_eq!(Topic::DsPipelineInjectionMetricCreated.name(), "ds.pipeline.injection.metric.created.v1");

    // DS Pipeline Transform Task Events
    assert_eq!(Topic::DsPipelineTransformTaskCompleted.name(), "ds.pipeline.transform.task.completed.v1");
    assert_eq!(Topic::DsPipelineTransformTaskFailed.name(), "ds.pipeline.transform.task.failed.v1");
    assert_eq!(Topic::DsPipelineTransformMetricCreated.name(), "ds.pipeline.transform.metric.created.v1");

    // DS Pipeline Migrator Task Events
    assert_eq!(Topic::DsPipelineMigratorTaskCompleted.name(), "ds.pipeline.migrator.task.completed.v1");
    assert_eq!(Topic::DsPipelineMigratorTaskFailed.name(), "ds.pipeline.migrator.task.failed.v1");
    assert_eq!(Topic::DsPipelineMigratorMetricCreated.name(), "ds.pipeline.migrator.metric.created.v1");

    // DS Pipeline Synchronizer Task Events
    assert_eq!(Topic::DsPipelineSynchronizerTaskRequested.name(), "ds.pipeline.synchronizer.task.requested.v1");
    assert_eq!(Topic::DsPipelineSynchronizerTaskCompleted.name(), "ds.pipeline.synchronizer.task.completed.v1");
    assert_eq!(Topic::DsPipelineSynchronizerTaskFailed.name(), "ds.pipeline.synchronizer.task.failed.v1");
    assert_eq!(Topic::DsPipelineSynchronizerMetricCreated.name(), "ds.pipeline.synchronizer.metric.created.v1");

    // DS Pipeline Synchronizer Job Events
    assert_eq!(Topic::DsPipelineSynchronizerJobRequested.name(), "ds.pipeline.synchronizer.job.requested.v1");
    assert_eq!(Topic::DsPipelineSynchronizerJobCompleted.name(), "ds.pipeline.synchronizer.job.completed.v1");
    assert_eq!(Topic::DsPipelineSynchronizerJobFailed.name(), "ds.pipeline.synchronizer.job.failed.v1");
    assert_eq!(Topic::DsPipelineSynchronizerJobMetricCreated.name(), "ds.pipeline.synchronizer.metric.created.v1");

    // DS Pipeline Clone Task Events
    assert_eq!(Topic::DsPipelineCloneTaskRequested.name(), "ds.pipeline.clone.task.requested.v1");
    assert_eq!(Topic::DsPipelineCloneTaskCompleted.name(), "ds.pipeline.clone.task.completed.v1");
    assert_eq!(Topic::DsPipelineCloneTaskFailed.name(), "ds.pipeline.clone.task.failed.v1");
    assert_eq!(Topic::DsPipelineCloneMetricCreated.name(), "ds.pipeline.clone.metric.created.v1");

    // DS Workflow Pipeline Job Events
    assert_eq!(Topic::DsWorkflowPipelineJobRequested.name(), "ds.workflow.pipeline.job.requested.v1");
    assert_eq!(Topic::DsWorkflowPipelineJobQueued.name(), "ds.workflow.pipeline.job.queued.v1");
    assert_eq!(Topic::DsWorkflowPipelineJobStarted.name(), "ds.workflow.pipeline.job.started.v1");
    assert_eq!(Topic::DsWorkflowPipelineJobCompleted.name(), "ds.workflow.pipeline.job.completed.v1");
    assert_eq!(Topic::DsWorkflowPipelineJobFailed.name(), "ds.workflow.pipeline.job.failed.v1");

    // DS Workflow Pipeline Task Events
    assert_eq!(Topic::DsWorkflowPipelineTaskStarted.name(), "ds.workflow.pipeline.task.started.v1");
    assert_eq!(Topic::DsWorkflowPipelineTaskCompleted.name(), "ds.workflow.pipeline.task.completed.v1");
    assert_eq!(Topic::DsWorkflowPipelineTaskFailed.name(), "ds.workflow.pipeline.task.failed.v1");

    // DS Workflow Pipeline Events
    assert_eq!(Topic::DsWorkflowPipelineCreated.name(), "ds.workflow..pipeline.created.v1");
    assert_eq!(Topic::DsWorkflowPipelineUpdated.name(), "ds.workflow..pipeline.updated.v1");
    assert_eq!(Topic::DsWorkflowPipelineDeleted.name(), "ds.workflow..pipeline.deleted.v1");

    // DS Workflow Dataset Events
    assert_eq!(Topic::DsWorkflowDatasetCreated.name(), "ds.workflow..dataset.created.v1");
    assert_eq!(Topic::DsWorkflowDatasetUpdated.name(), "ds.workflow..dataset.updated.v1");
    assert_eq!(Topic::DsWorkflowDatasetDeleted.name(), "ds.workflow..dataset.deleted.v1");

    // DS Workflow Linked Service Events
    assert_eq!(Topic::DsWorkflowLinkedServiceCreated.name(), "ds.workflow..linked-service.created.v1");
    assert_eq!(Topic::DsWorkflowLinkedServiceUpdated.name(), "ds.workflow..linked-service.updated.v1");
    assert_eq!(Topic::DsWorkflowLinkedServiceDeleted.name(), "ds.workflow..linked-service.deleted.v1");

    // DS Core Provision Job Events
    assert_eq!(Topic::DsCoreProvisionJobRequested.name(), "ds.core.provision.job.requested.v1");
    assert_eq!(Topic::DsCoreProvisionJobCompleted.name(), "ds.core.provision.job.completed.v1");
    assert_eq!(Topic::DsCoreProvisionJobFailed.name(), "ds.core.provision.job.failed.v1");

    // DS Core Config Events
    assert_eq!(Topic::DsCoreConfigInfoUpdated.name(), "ds.core.config.info.updated.v1");
    assert_eq!(Topic::DsCoreConfigStatusUpdated.name(), "ds.core.config.status.updated.v1");

    // DS Core Billing Events
    assert_eq!(Topic::DsCoreBillingUsageCreated.name(), "ds.core.billing.usage.created.v1");
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
