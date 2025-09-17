// topic enum
pub enum Topic {
    // pipeline job
    DsPipelineJobRequested,
    DsPipelineJobStarted,
    DsPipelineJobCompleted,
    DsPipelineJobFailed,

    // Pipeline Injection Task
    DsPipelineInjectionTaskRequested,
    DsPipelineInjectionTaskStarted,
    DsPipelineInjectionTaskCompleted,
    DsPipelineInjectionTaskFailed,

    // Pipeline Transformation Task
    DsPipelineTransformationTaskRequested,
    DsPipelineTransformationTaskStarted,
    DsPipelineTransformationTaskCompleted,
    DsPipelineTransformationTaskFailed,

    // Pipeline Synchronize Task
    DsPipelineSynchronizeTaskRequested,
    DsPipelineSynchronizeTaskStarted,
    DsPipelineSynchronizeTaskCompleted,
    DsPipelineSynchronizeTaskFailed,

    // Pipeline Synchronize Job
    DsPipelineSynchronizeJobRequested,
    DsPipelineSynchronizeJobStarted,
    DsPipelineSynchronizeJobCompleted,
    DsPipelineSynchronizeJobFailed,

    // Pipeline Clone Job
    DsPipelineCloneJobRequested,
    DsPipelineCloneJobStarted,
    DsPipelineCloneJobCompleted,
    DsPipelineCloneJobFailed,
}

impl Topic {
    pub fn name(&self) -> String {
        match self {
            Topic::DsPipelineJobRequested => "ds.pipeline.job.requested.v1".to_string(),
            Topic::DsPipelineJobStarted => "ds.pipeline.job.started.v1".to_string(),
            Topic::DsPipelineJobCompleted => "ds.pipeline.job.completed.v1".to_string(),
            Topic::DsPipelineJobFailed => "ds.pipeline.job.failed.v1".to_string(),
            Topic::DsPipelineInjectionTaskRequested => {
                "ds.pipeline.injection.task.requested.v1".to_string()
            }
            Topic::DsPipelineInjectionTaskStarted => {
                "ds.pipeline.injection.task.started.v1".to_string()
            }
            Topic::DsPipelineInjectionTaskCompleted => {
                "ds.pipeline.injection.task.completed.v1".to_string()
            }
            Topic::DsPipelineInjectionTaskFailed => {
                "ds.pipeline.injection.task.failed.v1".to_string()
            }
            Topic::DsPipelineTransformationTaskRequested => {
                "ds.pipeline.transformation.task.requested.v1".to_string()
            }
            Topic::DsPipelineTransformationTaskStarted => {
                "ds.pipeline.transformation.task.started.v1".to_string()
            }
            Topic::DsPipelineTransformationTaskCompleted => {
                "ds.pipeline.transformation.task.completed.v1".to_string()
            }
            Topic::DsPipelineTransformationTaskFailed => {
                "ds.pipeline.transformation.task.failed.v1".to_string()
            }
            Topic::DsPipelineSynchronizeTaskRequested => {
                "ds.pipeline.synchronize.task.requested.v1".to_string()
            }
            Topic::DsPipelineSynchronizeTaskStarted => {
                "ds.pipeline.synchronize.task.started.v1".to_string()
            }
            Topic::DsPipelineSynchronizeTaskCompleted => {
                "ds.pipeline.synchronize.task.completed.v1".to_string()
            }
            Topic::DsPipelineSynchronizeTaskFailed => {
                "ds.pipeline.synchronize.task.failed.v1".to_string()
            }
            Topic::DsPipelineSynchronizeJobRequested => {
                "ds.pipeline.synchronize.job.requested.v1".to_string()
            }
            Topic::DsPipelineSynchronizeJobStarted => {
                "ds.pipeline.synchronize.job.started.v1".to_string()
            }
            Topic::DsPipelineSynchronizeJobCompleted => {
                "ds.pipeline.synchronize.job.completed.v1".to_string()
            }
            Topic::DsPipelineSynchronizeJobFailed => {
                "ds.pipeline.synchronize.job.failed.v1".to_string()
            }
            Topic::DsPipelineCloneJobRequested => "ds.pipeline.clone.job.requested.v1".to_string(),
            Topic::DsPipelineCloneJobStarted => "ds.pipeline.clone.job.started.v1".to_string(),
            Topic::DsPipelineCloneJobCompleted => "ds.pipeline.clone.job.completed.v1".to_string(),
            Topic::DsPipelineCloneJobFailed => "ds.pipeline.clone.job.failed.v1".to_string(),
        }
    }
}
