use crate::errors::{ClientError, GetExperimentErrorCode};
use crate::{
    send_and_return_field, EmptyResponse, MlflowClient, Run, RunInfo, RunStatus, RunTag, ViewType,
};

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
struct SearchRunsQuery<'a, 'b, 'c, 'd> {
    experiment_ids: &'c [&'d str],
    filter: Option<&'a str>,
    run_view_type: Option<ViewType>,
    max_results: Option<u32>,
    order_by: Option<Vec<String>>,
    page_token: Option<&'b str>,
}

#[derive(serde::Deserialize, Debug)]
struct SearchRunsResponse {
    runs: Vec<Run>,
    next_page_token: Option<String>,
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

    /// Search for runs that satisfy expressions. Search expressions can use Metric and Param keys.
    pub fn search_runs(
        &self,
        experiment_ids: &[&str],
        filter: Option<&str>,
        run_view_type: Option<ViewType>,
        max_results: Option<u32>,
        order_by: Option<Vec<String>>,
        page_token: Option<&str>,
    ) -> Result<(Vec<Run>, Option<String>), ClientError<GetExperimentErrorCode>> {
        let req = self
            .client
            .post(&format!("{}/api/2.0/mlflow/runs/search", self.url))
            .json(&SearchRunsQuery {
                experiment_ids,
                filter,
                run_view_type,
                max_results,
                order_by,
                page_token,
            });
        send_and_return_field(req, |resp: SearchRunsResponse| {
            (resp.runs, resp.next_page_token)
        })
    }
}
