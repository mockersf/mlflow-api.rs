#![deny(
    warnings,
    missing_debug_implementations,
    missing_copy_implementations,
    trivial_casts,
    trivial_numeric_casts,
    unsafe_code,
    unstable_features,
    unused_import_braces,
    unused_qualifications,
    missing_docs
)]

//! MLFlow API Client.

mod structures;
pub use structures::*;
mod client;
pub use client::MlflowClient;
pub mod errors;

#[derive(serde::Deserialize, Debug)]
#[serde(untagged)]
enum Response<T, E: errors::ErrorCode + std::fmt::Debug + serde::Serialize> {
    Error(errors::ErrorResponse<E>),
    Success(T),
}

/// An error that can happen during an `Experiment` creation.
#[derive(serde::Deserialize, serde::Serialize, Debug, Copy, Clone, PartialEq)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum CreateExperimentErrorCode {
    /// An experiment with the same name already exists.
    ResourceAlreadyExists,
    /// Unknown error.
    #[serde(other)]
    UnknownError,
}
impl errors::ErrorCode for CreateExperimentErrorCode {}

#[derive(serde::Serialize, Debug)]
struct CreateExperimentQuery<'a, 'b> {
    name: &'a str,
    artifact_location: Option<&'b str>,
}

#[derive(serde::Deserialize, Debug)]
struct CreateExperimentResponse {
    experiment_id: String,
}

/// An error that can happen when listing `Experiment`s.
#[derive(serde::Deserialize, serde::Serialize, Debug, Clone, Copy, PartialEq)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum ListExperimentsErrorCode {
    /// Unknown error.
    #[serde(other)]
    UnknownError,
}
impl errors::ErrorCode for ListExperimentsErrorCode {}

#[derive(serde::Deserialize, Debug)]
struct ListExperimentsResponse {
    experiments: Vec<Experiment>,
}

/// An error that can happen when getting an `Experiment`.
#[derive(serde::Deserialize, serde::Serialize, Debug, Copy, Clone, PartialEq)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum GetExperimentErrorCode {
    /// An experiment with the requested ID could not be found.
    ResourceDoesNotExist,
    /// Unknown error.
    #[serde(other)]
    UnknownError,
}
impl errors::ErrorCode for GetExperimentErrorCode {}

#[derive(serde::Deserialize, Debug)]
struct GetExperimentResponse {
    experiment: Experiment,
}

#[derive(serde::Serialize, Debug)]
struct DeleteExperimentQuery<'a> {
    experiment_id: &'a str,
}

#[derive(serde::Deserialize, Debug)]
struct EmptyResponse {}

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
    ) -> Result<String, errors::ClientError<CreateExperimentErrorCode>> {
        let req = self
            .client
            .post(&format!("{}/api/2.0/mlflow/experiments/create", self.url))
            .json(&CreateExperimentQuery {
                name,
                artifact_location,
            });
        match req
            .send()?
            .json::<Response<CreateExperimentResponse, CreateExperimentErrorCode>>()?
        {
            Response::Success(resp) => Ok(resp.experiment_id),
            Response::Error(err) => Err(err.into()),
        }
    }

    /// Get a list of all experiments.
    pub fn list_experiments(
        &self,
        view_type: Option<ViewType>,
    ) -> Result<Vec<Experiment>, errors::ClientError<ListExperimentsErrorCode>> {
        let mut req = self
            .client
            .get(&format!("{}/api/2.0/mlflow/experiments/list", self.url));
        if let Some(view_type) = view_type {
            req = req.query(&[("view_type", view_type)]);
        }

        match req
            .send()?
            .json::<Response<ListExperimentsResponse, ListExperimentsErrorCode>>()?
        {
            Response::Success(resp) => Ok(resp.experiments),
            Response::Error(err) => Err(err.into()),
        }
    }

    /// Get metadata for an experiment. This method works on deleted experiments.
    pub fn get_experiment(
        &self,
        experiment_id: &str,
    ) -> Result<Experiment, errors::ClientError<GetExperimentErrorCode>> {
        let req = self
            .client
            .get(&format!("{}/api/2.0/mlflow/experiments/get", self.url))
            .query(&[("experiment_id", experiment_id)]);
        match req
            .send()?
            .json::<Response<GetExperimentResponse, GetExperimentErrorCode>>()?
        {
            Response::Success(resp) => Ok(resp.experiment),
            Response::Error(err) => Err(err.into()),
        }
    }

    /// Get metadata for an experiment. This endpoint will return deleted experiments, but prefers the active
    /// experiment if an active and deleted experiment share the same name. If multiple deleted experiments share the
    /// same name, the API will return one of them.
    pub fn get_experiment_by_name(
        &self,
        experiment_name: &str,
    ) -> Result<Experiment, errors::ClientError<GetExperimentErrorCode>> {
        let req = self
            .client
            .get(&format!(
                "{}/api/2.0/mlflow/experiments/get-by-name",
                self.url
            ))
            .query(&[("experiment_name", experiment_name)]);
        match req
            .send()?
            .json::<Response<GetExperimentResponse, GetExperimentErrorCode>>()?
        {
            Response::Success(resp) => Ok(resp.experiment),
            Response::Error(err) => Err(err.into()),
        }
    }

    /// Mark an experiment and associated metadata, runs, metrics, params, and tags for deletion. If the experiment
    /// uses FileStore, artifacts associated with experiment are also deleted.
    pub fn delete_experiment(
        &self,
        experiment_id: &str,
    ) -> Result<(), errors::ClientError<GetExperimentErrorCode>> {
        let req = self
            .client
            .post(&format!("{}/api/2.0/mlflow/experiments/delete", self.url))
            .json(&DeleteExperimentQuery { experiment_id });
        match req
            .send()?
            .json::<Response<EmptyResponse, GetExperimentErrorCode>>()?
        {
            Response::Success(_resp) => Ok(()),
            Response::Error(err) => Err(err.into()),
        }
    }

    /// Restore an experiment marked for deletion. This also restores associated metadata, runs, metrics, params, and
    /// tags. If experiment uses FileStore, underlying artifacts associated with experiment are also restored.
    pub fn restore_experiment(
        &self,
        experiment_id: &str,
    ) -> Result<(), errors::ClientError<GetExperimentErrorCode>> {
        let req = self
            .client
            .post(&format!("{}/api/2.0/mlflow/experiments/restore", self.url))
            .json(&RestoreExperimentQuery { experiment_id });
        match req
            .send()?
            .json::<Response<EmptyResponse, GetExperimentErrorCode>>()?
        {
            Response::Success(_resp) => Ok(()),
            Response::Error(err) => Err(err.into()),
        }
    }

    /// Update experiment metadata.
    pub fn update_experiment(
        &self,
        experiment_id: &str,
        new_name: &str,
    ) -> Result<(), errors::ClientError<GetExperimentErrorCode>> {
        let req = self
            .client
            .post(&format!("{}/api/2.0/mlflow/experiments/update", self.url))
            .json(&UpdateExperimentQuery {
                experiment_id,
                new_name,
            });
        match req
            .send()?
            .json::<Response<EmptyResponse, GetExperimentErrorCode>>()?
        {
            Response::Success(_resp) => Ok(()),
            Response::Error(err) => Err(err.into()),
        }
    }

    /// Set a tag on an experiment. Experiment tags are metadata that can be updated.
    pub fn set_experiment_tag(
        &self,
        experiment_id: &str,
        key: &str,
        value: &str,
    ) -> Result<(), errors::ClientError<GetExperimentErrorCode>> {
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
        match req
            .send()?
            .json::<Response<EmptyResponse, GetExperimentErrorCode>>()?
        {
            Response::Success(_resp) => Ok(()),
            Response::Error(err) => Err(err.into()),
        }
    }
}
