use crate::errors::{ClientError, GetExperimentErrorCode};
use crate::{send_and_return_field, EmptyResponse, MlflowClient};

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
