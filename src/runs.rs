use crate::errors::{ClientError, GetExperimentErrorCode};
use crate::{send_and_return_field, EmptyResponse, MlflowClient, Run, RunInfo, RunStatus, RunTag};

#[derive(serde::Serialize, Debug)]
struct CreateRunQuery<'a> {
    experiment_id: &'a str,
    start_time: Option<u64>,
    tags: Option<Vec<RunTag>>,
}

#[derive(serde::Deserialize, Debug)]
struct CreateRunResponse {
    run: Run,
}

#[derive(serde::Deserialize, Debug)]
struct GetRunResponse {
    run: Run,
}

#[derive(serde::Serialize, Debug)]
struct DeleteRunQuery<'a> {
    run_id: &'a str,
}

#[derive(serde::Serialize, Debug)]
struct RestoreRunQuery<'a> {
    run_id: &'a str,
}

#[derive(serde::Serialize, Debug)]
struct UpdateRunQuery<'a> {
    run_id: &'a str,
    status: RunStatus,
    end_time: Option<u64>,
}

#[derive(serde::Deserialize, Debug)]
struct UpdateRunResponse {
    run_info: RunInfo,
}

#[derive(serde::Serialize, Debug)]
struct SetRunTagQuery<'a, 'b, 'c> {
    run_id: &'a str,
    key: &'b str,
    value: &'c str,
}

#[derive(serde::Serialize, Debug)]
struct DeleteRunTagQuery<'a, 'b> {
    run_id: &'a str,
    key: &'b str,
}

impl MlflowClient {
    /// Create a new run within an experiment. A run is usually a single execution of a machine learning or data ETL
    /// pipeline. MLflow uses runs to track `Param`, `Metric`, and `RunTag` associated with a single execution.
    pub fn create_run(
        &self,
        experiment_id: &str,
        start_time: Option<u64>,
        tags: Option<Vec<RunTag>>,
    ) -> Result<Run, ClientError<GetExperimentErrorCode>> {
        let req = self
            .client
            .post(&format!("{}/api/2.0/mlflow/runs/create", self.url))
            .json(&CreateRunQuery {
                experiment_id,
                start_time,
                tags,
            });
        send_and_return_field(req, |resp: CreateRunResponse| resp.run)
    }

    /// Mark a run for deletion.
    pub fn delete_run(&self, run_id: &str) -> Result<(), ClientError<GetExperimentErrorCode>> {
        let req = self
            .client
            .post(&format!("{}/api/2.0/mlflow/runs/delete", self.url))
            .json(&DeleteRunQuery { run_id });
        send_and_return_field(req, |_: EmptyResponse| ())
    }

    /// Restore a deleted run.
    pub fn restore_run(&self, run_id: &str) -> Result<(), ClientError<GetExperimentErrorCode>> {
        let req = self
            .client
            .post(&format!("{}/api/2.0/mlflow/runs/restore", self.url))
            .json(&RestoreRunQuery { run_id });
        send_and_return_field(req, |_: EmptyResponse| ())
    }

    /// Get metadata, metrics, params, and tags for a run. In the case where multiple metrics with the same key are
    /// logged for a run, return only the value with the latest timestamp. If there are multiple values with the latest
    /// timestamp, return the maximum of these values.
    pub fn get_run(&self, run_id: &str) -> Result<Run, ClientError<GetExperimentErrorCode>> {
        let req = self
            .client
            .get(&format!("{}/api/2.0/mlflow/runs/get", self.url))
            .query(&[("run_id", run_id)]);
        send_and_return_field(req, |resp: GetRunResponse| resp.run)
    }

    /// Restore a deleted run.
    pub fn update_run(
        &self,
        run_id: &str,
        status: RunStatus,
        end_time: Option<u64>,
    ) -> Result<RunInfo, ClientError<GetExperimentErrorCode>> {
        let req = self
            .client
            .post(&format!("{}/api/2.0/mlflow/runs/update", self.url))
            .json(&UpdateRunQuery {
                run_id,
                status,
                end_time,
            });
        send_and_return_field(req, |resp: UpdateRunResponse| resp.run_info)
    }

    /// Set a tag on a run. Tags are run metadata that can be updated during a run and after a run completes.
    pub fn set_run_tag(
        &self,
        run_id: &str,
        key: &str,
        value: &str,
    ) -> Result<(), ClientError<GetExperimentErrorCode>> {
        let req = self
            .client
            .post(&format!("{}/api/2.0/mlflow/runs/set-tag", self.url))
            .json(&SetRunTagQuery { run_id, key, value });
        send_and_return_field(req, |_: EmptyResponse| ())
    }

    /// Delete a tag on a run. Tags are run metadata that can be updated during a run and after a run completes.
    pub fn delete_run_tag(
        &self,
        run_id: &str,
        key: &str,
    ) -> Result<(), ClientError<GetExperimentErrorCode>> {
        let req = self
            .client
            .post(&format!("{}/api/2.0/mlflow/runs/delete-tag", self.url))
            .json(&DeleteRunTagQuery { run_id, key });
        send_and_return_field(req, |_: EmptyResponse| ())
    }
}
