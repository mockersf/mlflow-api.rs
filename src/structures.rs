use serde::{Deserialize, Serialize};

/// Experiment
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Experiment {
    /// Unique identifier for the experiment.
    pub experiment_id: String,
    /// Human readable name that identifies the experiment.
    pub name: String,
    /// Location where artifacts for the experiment are stored.
    pub artifact_location: String,
    /// Current life cycle stage of the experiment: “active” or “deleted”. Deleted experiments are not returned by APIs.
    pub lifecycle_stage: LifecycleStage,
    /// Last update time
    pub last_update_time: Option<i64>,
    /// Creation time
    pub creation_time: Option<i64>,
    /// Additional metadata key-value pairs.
    pub tags: Option<Vec<ExperimentTag>>,
}

/// Tag for an experiment.
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ExperimentTag {
    /// The tag key.
    pub key: String,
    /// The tag value.
    pub value: String,
}

/// Metadata of a single artifact file or directory.
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct FileInfo {
    /// Path relative to the root artifact directory run.
    pub path: String,
    /// Whether the path is a directory.
    pub is_dir: bool,
    /// Size in bytes. Unset for directories.
    pub file_size: u64,
}

/// Metric associated with a run, represented as a key-value pair.
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Metric {
    /// Key identifying this metric.
    pub key: String,
    /// Value associated with this metric.
    pub value: f32,
    /// The timestamp at which this metric was recorded.
    pub timestamp: u64,
    /// Step at which to log the metric.
    pub step: u64,
}

/// Param associated with a run.
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Param {
    /// Key identifying this param.
    pub key: String,
    /// Value associated with this param.
    pub value: String,
}

/// A single run.
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Run {
    /// Run metadata.
    pub info: RunInfo,
    /// Run data.
    pub data: RunData,
}

/// Run data (metrics, params, and tags).
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct RunData {
    /// Run metrics.
    pub metrics: Vec<Metric>,
    /// Run parameters.
    pub params: Vec<Param>,
    /// Additional metadata key-value pairs.
    pub tags: Vec<RunTag>,
}

/// Metadata of a single run.
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct RunInfo {
    /// Unique identifier for the run.
    pub run_id: String,
    // /// [Deprecated, use run_id instead] Unique identifier for the run. This field will be removed in a future MLflow
    // /// version.
    // pub run_uuid: String,
    /// The experiment ID.
    pub experiment_id: String,
    /// User who initiated the run. This field is deprecated as of MLflow 1.0, and will be removed in a future MLflow
    /// release. Use ‘mlflow.user’ tag instead.
    pub user_id: String,
    /// Current status of the run.
    pub status: RunStatus,
    /// Unix timestamp of when the run started in milliseconds.
    pub start_time: u64,
    /// Unix timestamp of when the run ended in milliseconds.
    pub end_time: u64,
    /// URI of the directory where artifacts should be uploaded. This can be a local path (starting with “/”), or a
    /// distributed file system (DFS) path, like s3://bucket/directory or dbfs:/my/directory. If not set, the local
    /// ./mlruns directory is chosen.
    pub artifact_uri: String,
    /// Current life cycle stage of the experiment
    pub lifecycle_stage: LifecycleStage,
}

/// Life cycle stage of a experiment.
#[derive(Debug, Serialize, Deserialize, Clone, Copy)]
#[serde(rename_all = "camelCase")]
pub enum LifecycleStage {
    /// Run is active.
    Active,
    /// Run is deleted.
    Deleted,
}

/// Tag for a run.
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct RunTag {
    /// The tag key.
    pub key: String,
    /// The tag value.
    pub value: String,
}

/// Status of a run.
#[derive(Debug, Serialize, Deserialize, Clone, Copy)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum RunStatus {
    /// Run has been initiated.
    Running,
    /// Run is scheduled to run at a later time.
    Scheduled,
    /// Run has completed.
    Finished,
    /// Run execution failed.
    Failed,
    /// Run killed by user.
    Killed,
}

/// View type for ListExperiments query.
#[derive(Debug, Serialize, Deserialize, Clone, Copy)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum ViewType {
    /// Default. Return only active experiments.
    ActiveOnly,
    /// Return only deleted experiments.
    DeletedOnly,
    /// Get all experiments.
    All,
}
