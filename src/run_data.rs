use crate::errors::{ClientError, GetExperimentErrorCode};
use crate::{send_and_return_field, EmptyResponse, Metric, MlflowClient, Param, RunTag};

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

#[derive(serde::Serialize, Debug)]
struct LogMetricQuery<'a, 'b> {
    run_id: &'a str,
    key: &'b str,
    value: f32,
    timestamp: u64,
    step: Option<u64>,
}

#[derive(serde::Deserialize, Debug)]
struct GetMetricHistoryResponse {
    metrics: Vec<Metric>,
}

#[derive(serde::Serialize, Debug)]
struct LogParamQuery<'a, 'b, 'c> {
    run_id: &'a str,
    key: &'b str,
    value: &'c str,
}

#[derive(serde::Serialize, Debug)]
struct LogBatchQuery<'a, 'b, 'c, 'd, 'e, 'f, 'g> {
    run_id: &'a str,
    metrics: Option<&'b [&'c Metric]>,
    params: Option<&'d [&'e Param]>,
    tags: Option<&'f [&'g RunTag]>,
}

impl MlflowClient {
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

    /// Log a metric for a run. A metric is a key-value pair (string key, float value) with an associated timestamp.
    /// Examples include the various metrics that represent ML model accuracy. A metric can be logged multiple times.
    pub fn log_metric(
        &self,
        run_id: &str,
        key: &str,
        value: f32,
        timestamp: u64,
        step: Option<u64>,
    ) -> Result<(), ClientError<GetExperimentErrorCode>> {
        let req = self
            .client
            .post(&format!("{}/api/2.0/mlflow/runs/log-metric", self.url))
            .json(&LogMetricQuery {
                run_id,
                key,
                value,
                timestamp,
                step,
            });
        send_and_return_field(req, |_: EmptyResponse| ())
    }

    /// Get a list of all values for the specified metric for a given run.
    pub fn get_metric_history(
        &self,
        run_id: &str,
        metric_key: &str,
    ) -> Result<Vec<Metric>, ClientError<GetExperimentErrorCode>> {
        let req = self
            .client
            .get(&format!("{}/api/2.0/mlflow/metrics/get-history", self.url))
            .query(&[("run_id", run_id), ("metric_key", metric_key)]);
        send_and_return_field(req, |resp: GetMetricHistoryResponse| resp.metrics)
    }

    /// Log a param used for a run. A param is a key-value pair (string key, string value). Examples include
    /// hyperparameters used for ML model training and constant dates and values used in an ETL pipeline. A param can
    /// be logged only once for a run.
    pub fn log_param(
        &self,
        run_id: &str,
        key: &str,
        value: &str,
    ) -> Result<(), ClientError<GetExperimentErrorCode>> {
        let req = self
            .client
            .post(&format!("{}/api/2.0/mlflow/runs/log-parameter", self.url))
            .json(&LogParamQuery { run_id, key, value });
        send_and_return_field(req, |_: EmptyResponse| ())
    }

    /// Log a batch of metrics, params, and tags for a run. If any data failed to be persisted, the server will respond
    /// with an error (non-200 status code). In case of error (due to internal server error or an invalid request),
    /// partial data may be written. You can write metrics, params, and tags in interleaving fashion, but within a given
    /// entity type are guaranteed to follow the order specified in the request body.
    pub fn log_batch(
        &self,
        run_id: &str,
        metrics: Option<&[&Metric]>,
        params: Option<&[&Param]>,
        tags: Option<&[&RunTag]>,
    ) -> Result<(), ClientError<GetExperimentErrorCode>> {
        let req = self
            .client
            .post(&format!("{}/api/2.0/mlflow/runs/log-batch", self.url))
            .json(&LogBatchQuery {
                run_id,
                metrics,
                params,
                tags,
            });
        send_and_return_field(req, |_: EmptyResponse| ())
    }
}
