use crate::errors::{
    ClientError, CreateExperimentErrorCode, GetExperimentErrorCode, ListExperimentsErrorCode,
};
use crate::{send_and_return_field, EmptyResponse, Experiment, MlflowClient, ViewType};

#[derive(serde::Serialize, Debug)]
struct CreateExperimentQuery<'a, 'b> {
    name: &'a str,
    artifact_location: Option<&'b str>,
}

#[derive(serde::Deserialize, Debug)]
struct CreateExperimentResponse {
    experiment_id: String,
}

#[derive(serde::Deserialize, Debug)]
struct ListExperimentsResponse {
    experiments: Vec<Experiment>,
}

#[derive(serde::Deserialize, Debug)]
struct GetExperimentResponse {
    experiment: Experiment,
}

#[derive(serde::Serialize, Debug)]
struct DeleteExperimentQuery<'a> {
    experiment_id: &'a str,
}

#[derive(serde::Serialize, Debug)]
struct RestoreExperimentQuery<'a> {
    experiment_id: &'a str,
}

#[derive(serde::Serialize, Debug)]
struct UpdateExperimentQuery<'a, 'b> {
    experiment_id: &'a str,
    new_name: &'b str,
}

#[derive(serde::Serialize, Debug)]
struct SetExperimentTagQuery<'a, 'b, 'c> {
    experiment_id: &'a str,
    key: &'b str,
    value: &'c str,
}

impl MlflowClient {
    /// Create an experiment with a name. Returns the ID of the newly created experiment. Validates that another
    /// experiment with the same name does not already exist and fails if another experiment with the same name
    /// already exists.
    pub fn create_experiment(
        &self,
        name: &str,
        artifact_location: Option<&str>,
    ) -> Result<String, ClientError<CreateExperimentErrorCode>> {
        let req = self
            .client
            .post(&format!("{}/api/2.0/mlflow/experiments/create", self.url))
            .json(&CreateExperimentQuery {
                name,
                artifact_location,
            });
        send_and_return_field(req, |resp: CreateExperimentResponse| resp.experiment_id)
    }

    /// Get a list of all experiments.
    pub fn list_experiments(
        &self,
        view_type: Option<ViewType>,
    ) -> Result<Vec<Experiment>, ClientError<ListExperimentsErrorCode>> {
        let mut req = self
            .client
            .get(&format!("{}/api/2.0/mlflow/experiments/list", self.url));
        if let Some(view_type) = view_type {
            req = req.query(&[("view_type", view_type)]);
        }
        send_and_return_field(req, |resp: ListExperimentsResponse| resp.experiments)
    }

    /// Get metadata for an experiment. This method works on deleted experiments.
    pub fn get_experiment(
        &self,
        experiment_id: &str,
    ) -> Result<Experiment, ClientError<GetExperimentErrorCode>> {
        let req = self
            .client
            .get(&format!("{}/api/2.0/mlflow/experiments/get", self.url))
            .query(&[("experiment_id", experiment_id)]);
        send_and_return_field(req, |resp: GetExperimentResponse| resp.experiment)
    }

    /// Get metadata for an experiment. This endpoint will return deleted experiments, but prefers the active
    /// experiment if an active and deleted experiment share the same name. If multiple deleted experiments share the
    /// same name, the API will return one of them.
    pub fn get_experiment_by_name(
        &self,
        experiment_name: &str,
    ) -> Result<Experiment, ClientError<GetExperimentErrorCode>> {
        let req = self
            .client
            .get(&format!(
                "{}/api/2.0/mlflow/experiments/get-by-name",
                self.url
            ))
            .query(&[("experiment_name", experiment_name)]);
        send_and_return_field(req, |resp: GetExperimentResponse| resp.experiment)
    }

    /// Mark an experiment and associated metadata, runs, metrics, params, and tags for deletion. If the experiment
    /// uses FileStore, artifacts associated with experiment are also deleted.
    pub fn delete_experiment(
        &self,
        experiment_id: &str,
    ) -> Result<(), ClientError<GetExperimentErrorCode>> {
        let req = self
            .client
            .post(&format!("{}/api/2.0/mlflow/experiments/delete", self.url))
            .json(&DeleteExperimentQuery { experiment_id });
        send_and_return_field(req, |_: EmptyResponse| ())
    }

    /// Restore an experiment marked for deletion. This also restores associated metadata, runs, metrics, params, and
    /// tags. If experiment uses FileStore, underlying artifacts associated with experiment are also restored.
    pub fn restore_experiment(
        &self,
        experiment_id: &str,
    ) -> Result<(), ClientError<GetExperimentErrorCode>> {
        let req = self
            .client
            .post(&format!("{}/api/2.0/mlflow/experiments/restore", self.url))
            .json(&RestoreExperimentQuery { experiment_id });
        send_and_return_field(req, |_: EmptyResponse| ())
    }

    /// Update experiment metadata.
    pub fn update_experiment(
        &self,
        experiment_id: &str,
        new_name: &str,
    ) -> Result<(), ClientError<GetExperimentErrorCode>> {
        let req = self
            .client
            .post(&format!("{}/api/2.0/mlflow/experiments/update", self.url))
            .json(&UpdateExperimentQuery {
                experiment_id,
                new_name,
            });
        send_and_return_field(req, |_: EmptyResponse| ())
    }

    /// Set a tag on an experiment. Experiment tags are metadata that can be updated.
    pub fn set_experiment_tag(
        &self,
        experiment_id: &str,
        key: &str,
        value: &str,
    ) -> Result<(), ClientError<GetExperimentErrorCode>> {
        let req = self
            .client
            .post(&format!(
                "{}/api/2.0/mlflow/experiments/set-experiment-tag",
                self.url
            ))
            .json(&SetExperimentTagQuery {
                experiment_id,
                key,
                value,
            });
        send_and_return_field(req, |_: EmptyResponse| ())
    }
}
