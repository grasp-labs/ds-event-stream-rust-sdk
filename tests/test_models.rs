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
    assert_eq!(Topic::IdpIdentityUserCreated.to_string(), "idp.identity..user.created.v1");
    assert_eq!(Topic::IdpIdentityUserUpdated.to_string(), "idp.identity..user.updated.v1");
    assert_eq!(Topic::IdpIdentityUserDeleted.to_string(), "idp.identity..user.deleted.v1");
    assert_eq!(Topic::IdpIdentityUserAuthenticated.to_string(), "idp.identity..user.authenticated.v1");

    // IDP Identity Tenant Events
    assert_eq!(Topic::IdpIdentityTenantCreated.to_string(), "idp.identity..tenant.created.v1");
    assert_eq!(Topic::IdpIdentityTenantUpdated.to_string(), "idp.identity..tenant.updated.v1");
    assert_eq!(Topic::IdpIdentityTenantDeleted.to_string(), "idp.identity..tenant.deleted.v1");

    // DS Pipeline Job Events
    assert_eq!(Topic::DsPipelineJobRequested.to_string(), "ds.pipeline..job.requested.v1");
    assert_eq!(Topic::DsPipelineJobStarted.to_string(), "ds.pipeline..job.started.v1");
    assert_eq!(Topic::DsPipelineJobCompleted.to_string(), "ds.pipeline..job.completed.v1");
    assert_eq!(Topic::DsPipelineJobFailed.to_string(), "ds.pipeline..job.failed.v1");

    // DS Pipeline Injection Task Events
    assert_eq!(Topic::DsPipelineInjectionTaskCompleted.to_string(), "ds.pipeline.injection.task.completed.v1");
    assert_eq!(Topic::DsPipelineInjectionTaskFailed.to_string(), "ds.pipeline.injection.task.failed.v1");
    assert_eq!(Topic::DsPipelineInjectionMetricCreated.to_string(), "ds.pipeline.injection.metric.created.v1");

    // DS Pipeline Transform Task Events
    assert_eq!(Topic::DsPipelineTransformTaskCompleted.to_string(), "ds.pipeline.transform.task.completed.v1");
    assert_eq!(Topic::DsPipelineTransformTaskFailed.to_string(), "ds.pipeline.transform.task.failed.v1");
    assert_eq!(Topic::DsPipelineTransformMetricCreated.to_string(), "ds.pipeline.transform.metric.created.v1");

    // DS Pipeline Migrator Task Events
    assert_eq!(Topic::DsPipelineMigratorTaskCompleted.to_string(), "ds.pipeline.migrator.task.completed.v1");
    assert_eq!(Topic::DsPipelineMigratorTaskFailed.to_string(), "ds.pipeline.migrator.task.failed.v1");
    assert_eq!(Topic::DsPipelineMigratorMetricCreated.to_string(), "ds.pipeline.migrator.metric.created.v1");

    // DS Pipeline Synchronizer Task Events
    assert_eq!(Topic::DsPipelineSynchronizerTaskRequested.to_string(), "ds.pipeline.synchronizer.task.requested.v1");
    assert_eq!(Topic::DsPipelineSynchronizerTaskCompleted.to_string(), "ds.pipeline.synchronizer.task.completed.v1");
    assert_eq!(Topic::DsPipelineSynchronizerTaskFailed.to_string(), "ds.pipeline.synchronizer.task.failed.v1");
    assert_eq!(Topic::DsPipelineSynchronizerMetricCreated.to_string(), "ds.pipeline.synchronizer.metric.created.v1");

    // DS Pipeline Synchronizer Job Events
    assert_eq!(Topic::DsPipelineSynchronizerJobRequested.to_string(), "ds.pipeline.synchronizer.job.requested.v1");
    assert_eq!(Topic::DsPipelineSynchronizerJobCompleted.to_string(), "ds.pipeline.synchronizer.job.completed.v1");
    assert_eq!(Topic::DsPipelineSynchronizerJobFailed.to_string(), "ds.pipeline.synchronizer.job.failed.v1");
    assert_eq!(Topic::DsPipelineSynchronizerMetricCreated.to_string(), "ds.pipeline.synchronizer.metric.created.v1");

    // DS Pipeline Clone Task Events
    assert_eq!(Topic::DsPipelineCloneTaskRequested.to_string(), "ds.pipeline.clone.task.requested.v1");
    assert_eq!(Topic::DsPipelineCloneTaskCompleted.to_string(), "ds.pipeline.clone.task.completed.v1");
    assert_eq!(Topic::DsPipelineCloneTaskFailed.to_string(), "ds.pipeline.clone.task.failed.v1");
    assert_eq!(Topic::DsPipelineCloneMetricCreated.to_string(), "ds.pipeline.clone.metric.created.v1");

    // DS Workflow Pipeline Job Events
    assert_eq!(Topic::DsWorkflowPipelineJobRequested.to_string(), "ds.workflow.pipeline.job.requested.v1");
    assert_eq!(Topic::DsWorkflowPipelineJobQueued.to_string(), "ds.workflow.pipeline.job.queued.v1");
    assert_eq!(Topic::DsWorkflowPipelineJobStarted.to_string(), "ds.workflow.pipeline.job.started.v1");
    assert_eq!(Topic::DsWorkflowPipelineJobCompleted.to_string(), "ds.workflow.pipeline.job.completed.v1");
    assert_eq!(Topic::DsWorkflowPipelineJobFailed.to_string(), "ds.workflow.pipeline.job.failed.v1");

    // DS Workflow Pipeline Task Events
    assert_eq!(Topic::DsWorkflowPipelineTaskStarted.to_string(), "ds.workflow.pipeline.task.started.v1");
    assert_eq!(Topic::DsWorkflowPipelineTaskCompleted.to_string(), "ds.workflow.pipeline.task.completed.v1");
    assert_eq!(Topic::DsWorkflowPipelineTaskFailed.to_string(), "ds.workflow.pipeline.task.failed.v1");

    // DS Workflow Pipeline Events
    assert_eq!(Topic::DsWorkflowPipelineCreated.to_string(), "ds.workflow..pipeline.created.v1");
    assert_eq!(Topic::DsWorkflowPipelineUpdated.to_string(), "ds.workflow..pipeline.updated.v1");
    assert_eq!(Topic::DsWorkflowPipelineDeleted.to_string(), "ds.workflow..pipeline.deleted.v1");

    // DS Workflow Dataset Events
    assert_eq!(Topic::DsWorkflowDatasetCreated.to_string(), "ds.workflow..dataset.created.v1");
    assert_eq!(Topic::DsWorkflowDatasetUpdated.to_string(), "ds.workflow..dataset.updated.v1");
    assert_eq!(Topic::DsWorkflowDatasetDeleted.to_string(), "ds.workflow..dataset.deleted.v1");

    // DS Workflow Linked Service Events
    assert_eq!(Topic::DsWorkflowLinkedServiceCreated.to_string(), "ds.workflow..linked-service.created.v1");
    assert_eq!(Topic::DsWorkflowLinkedServiceUpdated.to_string(), "ds.workflow..linked-service.updated.v1");
    assert_eq!(Topic::DsWorkflowLinkedServiceDeleted.to_string(), "ds.workflow..linked-service.deleted.v1");

    // DS Core Provision Job Events
    assert_eq!(Topic::DsCoreProvisionJobRequested.to_string(), "ds.core.provision.job.requested.v1");
    assert_eq!(Topic::DsCoreProvisionJobCompleted.to_string(), "ds.core.provision.job.completed.v1");
    assert_eq!(Topic::DsCoreProvisionJobFailed.to_string(), "ds.core.provision.job.failed.v1");

    // DS Core Config Events
    assert_eq!(Topic::DsCoreConfigInfoUpdated.to_string(), "ds.core.config.info.updated.v1");
    assert_eq!(Topic::DsCoreConfigStatusUpdated.to_string(), "ds.core.config.status.updated.v1");

    // DS Core Billing Events
    assert_eq!(Topic::DsCoreBillingUsageCreated.to_string(), "ds.core.billing.usage.created.v1");
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
