//! Topics module.
//!
//! ## Overview
//! This module contains the topics for the DS Event Stream.
//!
//! ## Features
//! * Get the topics for the DS Event Stream.
//!
//! ### Example
//! ```
//! use ds_event_stream_rust_sdk::model::topics::Topic;
//!
//! let topic = Topic::DsPipelineJobRequested.name();
//! assert_eq!(topic, "ds.pipeline..job.requested.v1");
//! ```

///
/// This enum contains all the topics for the DS Event Stream.
///
/// # Topics
///
/// * `IdpIdentityUserCreated` - The event when an identity user is created.
/// * `IdpIdentityUserUpdated` - The event when an identity user is updated.
/// * `IdpIdentityUserDeleted` - The event when an identity user is deleted.
/// * `IdpIdentityUserAuthenticated` - The event when an identity user is authenticated.
/// * `IdpIdentityTenantCreated` - The event when an identity tenant is created.
/// * `IdpIdentityTenantUpdated` - The event when an identity tenant is updated.
/// * `IdpIdentityTenantDeleted` - The event when an identity tenant is deleted.
/// * `DsPipelineJobRequested` - The event when a pipeline job is requested.
/// * `DsPipelineJobStarted` - The event when a pipeline job is started.
/// * `DsPipelineJobCompleted` - The event when a pipeline job is completed.
/// * `DsPipelineJobFailed` - The event when a pipeline job is failed.
/// * `DsPipelineInjectionTaskCompleted` - The event when a pipeline injection task is completed.
/// * `DsPipelineInjectionTaskFailed` - The event when a pipeline injection task is failed.
/// * `DsPipelineInjectionMetricCreated` - The event when a pipeline injection metric is created.
/// * `DsPipelineTransformTaskCompleted` - The event when a pipeline transform task is completed.
/// * `DsPipelineTransformTaskFailed` - The event when a pipeline transform task is failed.
/// * `DsPipelineTransformMetricCreated` - The event when a pipeline transform metric is created.
/// * `DsPipelineMigratorTaskCompleted` - The event when a pipeline migrator task is completed.
/// * `DsPipelineMigratorTaskFailed` - The event when a pipeline migrator task is failed.
/// * `DsPipelineMigratorMetricCreated` - The event when a pipeline migrator metric is created.
/// * `DsPipelineSynchronizerTaskRequested` - The event when a pipeline synchronizer task is requested.
/// * `DsPipelineSynchronizerTaskCompleted` - The event when a pipeline synchronizer task is completed.
/// * `DsPipelineSynchronizerTaskFailed` - The event when a pipeline synchronizer task is failed.
/// * `DsPipelineSynchronizerMetricCreated` - The event when a pipeline synchronizer metric is created.
/// * `DsPipelineSynchronizerJobRequested` - The event when a pipeline synchronizer job is requested.
/// * `DsPipelineSynchronizerJobCompleted` - The event when a pipeline synchronizer job is completed.
/// * `DsPipelineSynchronizerJobFailed` - The event when a pipeline synchronizer job is failed.
/// * `DsPipelineSynchronizerJobMetricCreated` - The event when a pipeline synchronizer job metric is created.
/// * `DsPipelineCloneTaskRequested` - The event when a pipeline clone task is requested.
/// * `DsPipelineCloneTaskCompleted` - The event when a pipeline clone task is completed.
/// * `DsPipelineCloneTaskFailed` - The event when a pipeline clone task is failed.
/// * `DsPipelineCloneMetricCreated` - The event when a pipeline clone metric is created.
/// * `DsWorkflowPipelineJobRequested` - The event when a workflow pipeline job is requested.
/// * `DsWorkflowPipelineJobQueued` - The event when a workflow pipeline job is queued.
/// * `DsWorkflowPipelineJobStarted` - The event when a workflow pipeline job is started.
/// * `DsWorkflowPipelineJobCompleted` - The event when a workflow pipeline job is completed.
/// * `DsWorkflowPipelineJobFailed` - The event when a workflow pipeline job is failed.
/// * `DsWorkflowPipelineTaskStarted` - The event when a workflow pipeline task is started.
/// * `DsWorkflowPipelineTaskCompleted` - The event when a workflow pipeline task is completed.
/// * `DsWorkflowPipelineTaskFailed` - The event when a workflow pipeline task is failed.
/// * `DsWorkflowPipelineCreated` - The event when a workflow pipeline is created.
/// * `DsWorkflowPipelineUpdated` - The event when a workflow pipeline is updated.
/// * `DsWorkflowPipelineDeleted` - The event when a workflow pipeline is deleted.
/// * `DsWorkflowDatasetCreated` - The event when a workflow dataset is created.
/// * `DsWorkflowDatasetUpdated` - The event when a workflow dataset is updated.
/// * `DsWorkflowDatasetDeleted` - The event when a workflow dataset is deleted.
pub enum Topic {
    // IDP Identity User Events
    IdpIdentityUserCreated,
    IdpIdentityUserUpdated,
    IdpIdentityUserDeleted,
    IdpIdentityUserAuthenticated,

    // IDP Identity Tenant Events
    IdpIdentityTenantCreated,
    IdpIdentityTenantUpdated,
    IdpIdentityTenantDeleted,

    // DS Pipeline Job Events
    DsPipelineJobRequested,
    DsPipelineJobStarted,
    DsPipelineJobCompleted,
    DsPipelineJobFailed,

    // DS Pipeline Injection Task Events
    DsPipelineInjectionTaskCompleted,
    DsPipelineInjectionTaskFailed,
    DsPipelineInjectionMetricCreated,

    // DS Pipeline Transform Task Events
    DsPipelineTransformTaskCompleted,
    DsPipelineTransformTaskFailed,
    DsPipelineTransformMetricCreated,

    // DS Pipeline Migrator Task Events
    DsPipelineMigratorTaskCompleted,
    DsPipelineMigratorTaskFailed,
    DsPipelineMigratorMetricCreated,

    // DS Pipeline Synchronizer Task Events
    DsPipelineSynchronizerTaskRequested,
    DsPipelineSynchronizerTaskCompleted,
    DsPipelineSynchronizerTaskFailed,
    DsPipelineSynchronizerMetricCreated,

    // DS Pipeline Synchronizer Job Events
    DsPipelineSynchronizerJobRequested,
    DsPipelineSynchronizerJobCompleted,
    DsPipelineSynchronizerJobFailed,
    DsPipelineSynchronizerJobMetricCreated,

    // DS Pipeline Clone Task Events
    DsPipelineCloneTaskRequested,
    DsPipelineCloneTaskCompleted,
    DsPipelineCloneTaskFailed,
    DsPipelineCloneMetricCreated,

    // DS Workflow Pipeline Job Events
    DsWorkflowPipelineJobRequested,
    DsWorkflowPipelineJobQueued,
    DsWorkflowPipelineJobStarted,
    DsWorkflowPipelineJobCompleted,
    DsWorkflowPipelineJobFailed,

    // DS Workflow Pipeline Task Events
    DsWorkflowPipelineTaskStarted,
    DsWorkflowPipelineTaskCompleted,
    DsWorkflowPipelineTaskFailed,

    // DS Workflow Pipeline Events
    DsWorkflowPipelineCreated,
    DsWorkflowPipelineUpdated,
    DsWorkflowPipelineDeleted,

    // DS Workflow Dataset Events
    DsWorkflowDatasetCreated,
    DsWorkflowDatasetUpdated,
    DsWorkflowDatasetDeleted,

    // DS Workflow Linked Service Events
    DsWorkflowLinkedServiceCreated,
    DsWorkflowLinkedServiceUpdated,
    DsWorkflowLinkedServiceDeleted,

    // DS Core Provision Job Events
    DsCoreProvisionJobRequested,
    DsCoreProvisionJobCompleted,
    DsCoreProvisionJobFailed,

    // DS Core Config Events
    DsCoreConfigInfoUpdated,
    DsCoreConfigStatusUpdated,

    // DS Core Billing Events
    DsCoreBillingUsageCreated,
}

impl Topic {
    pub fn name(&self) -> String {
        match self {
            // IDP Identity User Events
            Topic::IdpIdentityUserCreated => "idp.identity..user.created.v1".to_string(),
            Topic::IdpIdentityUserUpdated => "idp.identity..user.updated.v1".to_string(),
            Topic::IdpIdentityUserDeleted => "idp.identity..user.deleted.v1".to_string(),
            Topic::IdpIdentityUserAuthenticated => "idp.identity..user.authenticated.v1".to_string(),

            // IDP Identity Tenant Events
            Topic::IdpIdentityTenantCreated => "idp.identity..tenant.created.v1".to_string(),
            Topic::IdpIdentityTenantUpdated => "idp.identity..tenant.updated.v1".to_string(),
            Topic::IdpIdentityTenantDeleted => "idp.identity..tenant.deleted.v1".to_string(),

            // DS Pipeline Job Events
            Topic::DsPipelineJobRequested => "ds.pipeline..job.requested.v1".to_string(),
            Topic::DsPipelineJobStarted => "ds.pipeline..job.started.v1".to_string(),
            Topic::DsPipelineJobCompleted => "ds.pipeline..job.completed.v1".to_string(),
            Topic::DsPipelineJobFailed => "ds.pipeline..job.failed.v1".to_string(),

            // DS Pipeline Injection Task Events
            Topic::DsPipelineInjectionTaskCompleted => "ds.pipeline.injection.task.completed.v1".to_string(),
            Topic::DsPipelineInjectionTaskFailed => "ds.pipeline.injection.task.failed.v1".to_string(),
            Topic::DsPipelineInjectionMetricCreated => "ds.pipeline.injection.metric.created.v1".to_string(),

            // DS Pipeline Transform Task Events
            Topic::DsPipelineTransformTaskCompleted => "ds.pipeline.transform.task.completed.v1".to_string(),
            Topic::DsPipelineTransformTaskFailed => "ds.pipeline.transform.task.failed.v1".to_string(),
            Topic::DsPipelineTransformMetricCreated => "ds.pipeline.transform.metric.created.v1".to_string(),

            // DS Pipeline Migrator Task Events
            Topic::DsPipelineMigratorTaskCompleted => "ds.pipeline.migrator.task.completed.v1".to_string(),
            Topic::DsPipelineMigratorTaskFailed => "ds.pipeline.migrator.task.failed.v1".to_string(),
            Topic::DsPipelineMigratorMetricCreated => "ds.pipeline.migrator.metric.created.v1".to_string(),

            // DS Pipeline Synchronizer Task Events
            Topic::DsPipelineSynchronizerTaskRequested => "ds.pipeline.synchronizer.task.requested.v1".to_string(),
            Topic::DsPipelineSynchronizerTaskCompleted => "ds.pipeline.synchronizer.task.completed.v1".to_string(),
            Topic::DsPipelineSynchronizerTaskFailed => "ds.pipeline.synchronizer.task.failed.v1".to_string(),
            Topic::DsPipelineSynchronizerMetricCreated => "ds.pipeline.synchronizer.metric.created.v1".to_string(),

            // DS Pipeline Synchronizer Job Events
            Topic::DsPipelineSynchronizerJobRequested => "ds.pipeline.synchronizer.job.requested.v1".to_string(),
            Topic::DsPipelineSynchronizerJobCompleted => "ds.pipeline.synchronizer.job.completed.v1".to_string(),
            Topic::DsPipelineSynchronizerJobFailed => "ds.pipeline.synchronizer.job.failed.v1".to_string(),
            Topic::DsPipelineSynchronizerJobMetricCreated => "ds.pipeline.synchronizer.metric.created.v1".to_string(),

            // DS Pipeline Clone Task Events
            Topic::DsPipelineCloneTaskRequested => "ds.pipeline.clone.task.requested.v1".to_string(),
            Topic::DsPipelineCloneTaskCompleted => "ds.pipeline.clone.task.completed.v1".to_string(),
            Topic::DsPipelineCloneTaskFailed => "ds.pipeline.clone.task.failed.v1".to_string(),
            Topic::DsPipelineCloneMetricCreated => "ds.pipeline.clone.metric.created.v1".to_string(),

            // DS Workflow Pipeline Job Events
            Topic::DsWorkflowPipelineJobRequested => "ds.workflow.pipeline.job.requested.v1".to_string(),
            Topic::DsWorkflowPipelineJobQueued => "ds.workflow.pipeline.job.queued.v1".to_string(),
            Topic::DsWorkflowPipelineJobStarted => "ds.workflow.pipeline.job.started.v1".to_string(),
            Topic::DsWorkflowPipelineJobCompleted => "ds.workflow.pipeline.job.completed.v1".to_string(),
            Topic::DsWorkflowPipelineJobFailed => "ds.workflow.pipeline.job.failed.v1".to_string(),

            // DS Workflow Pipeline Task Events
            Topic::DsWorkflowPipelineTaskStarted => "ds.workflow.pipeline.task.started.v1".to_string(),
            Topic::DsWorkflowPipelineTaskCompleted => "ds.workflow.pipeline.task.completed.v1".to_string(),
            Topic::DsWorkflowPipelineTaskFailed => "ds.workflow.pipeline.task.failed.v1".to_string(),

            // DS Workflow Pipeline Events
            Topic::DsWorkflowPipelineCreated => "ds.workflow..pipeline.created.v1".to_string(),
            Topic::DsWorkflowPipelineUpdated => "ds.workflow..pipeline.updated.v1".to_string(),
            Topic::DsWorkflowPipelineDeleted => "ds.workflow..pipeline.deleted.v1".to_string(),

            // DS Workflow Dataset Events
            Topic::DsWorkflowDatasetCreated => "ds.workflow..dataset.created.v1".to_string(),
            Topic::DsWorkflowDatasetUpdated => "ds.workflow..dataset.updated.v1".to_string(),
            Topic::DsWorkflowDatasetDeleted => "ds.workflow..dataset.deleted.v1".to_string(),

            // DS Workflow Linked Service Events
            Topic::DsWorkflowLinkedServiceCreated => "ds.workflow..linked-service.created.v1".to_string(),
            Topic::DsWorkflowLinkedServiceUpdated => "ds.workflow..linked-service.updated.v1".to_string(),
            Topic::DsWorkflowLinkedServiceDeleted => "ds.workflow..linked-service.deleted.v1".to_string(),

            // DS Core Provision Job Events
            Topic::DsCoreProvisionJobRequested => "ds.core.provision.job.requested.v1".to_string(),
            Topic::DsCoreProvisionJobCompleted => "ds.core.provision.job.completed.v1".to_string(),
            Topic::DsCoreProvisionJobFailed => "ds.core.provision.job.failed.v1".to_string(),

            // DS Core Config Events
            Topic::DsCoreConfigInfoUpdated => "ds.core.config.info.updated.v1".to_string(),
            Topic::DsCoreConfigStatusUpdated => "ds.core.config.status.updated.v1".to_string(),

            // DS Core Billing Events
            Topic::DsCoreBillingUsageCreated => "ds.core.billing.usage.created.v1".to_string(),
        }
    }
}
