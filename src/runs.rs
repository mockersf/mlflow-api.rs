use crate::errors::{ClientError, GetExperimentErrorCode};
use crate::{EmptyResponse, MlflowClient, Response, Run, RunInfo, RunStatus, RunTag};

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
        match req
            .send()?
            .json::<Response<CreateRunResponse, GetExperimentErrorCode>>()?
        {
            Response::Success(resp) => Ok(resp.run),
            Response::Error(err) => Err(err.into()),
        }
    }

    /// Mark a run for deletion.
    pub fn delete_run(&self, run_id: &str) -> Result<(), ClientError<GetExperimentErrorCode>> {
        let req = self
            .client
            .post(&format!("{}/api/2.0/mlflow/runs/delete", self.url))
            .json(&DeleteRunQuery { run_id });
        match req
            .send()?
            .json::<Response<EmptyResponse, GetExperimentErrorCode>>()?
        {
            Response::Success(_resp) => Ok(()),
            Response::Error(err) => Err(err.into()),
        }
    }

    /// Restore a deleted run.
    pub fn restore_run(&self, run_id: &str) -> Result<(), ClientError<GetExperimentErrorCode>> {
        let req = self
            .client
            .post(&format!("{}/api/2.0/mlflow/runs/restore", self.url))
            .json(&RestoreRunQuery { run_id });
        match req
            .send()?
            .json::<Response<EmptyResponse, GetExperimentErrorCode>>()?
        {
            Response::Success(_resp) => Ok(()),
            Response::Error(err) => Err(err.into()),
        }
    }

    /// Get metadata, metrics, params, and tags for a run. In the case where multiple metrics with the same key are
    /// logged for a run, return only the value with the latest timestamp. If there are multiple values with the latest
    /// timestamp, return the maximum of these values.
    pub fn get_run(&self, run_id: &str) -> Result<Run, ClientError<GetExperimentErrorCode>> {
        let req = self
            .client
            .get(&format!("{}/api/2.0/mlflow/runs/get", self.url))
            .query(&[("run_id", run_id)]);
        match req
            .send()?
            .json::<Response<GetRunResponse, GetExperimentErrorCode>>()?
        {
            Response::Success(resp) => Ok(resp.run),
            Response::Error(err) => Err(err.into()),
        }
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
        match req
            .send()?
            .json::<Response<UpdateRunResponse, GetExperimentErrorCode>>()?
        {
            Response::Success(resp) => Ok(resp.run_info),
            Response::Error(err) => Err(err.into()),
        }
    }
}
