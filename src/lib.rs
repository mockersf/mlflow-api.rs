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
    Success(T),
    Error(errors::ErrorResponse<E>),
}

/// An error that can happend during `Experiment` creation.
#[derive(serde::Deserialize, serde::Serialize, Debug, Copy, Clone)]
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
struct CreateExperimentQuery {
    name: String,
    artifact_location: Option<String>,
}

#[derive(serde::Deserialize, Debug)]
struct CreateExperimentResponse {
    experiment_id: String,
}

impl MlflowClient {
    /// Create an experiment with a name. Returns the ID of the newly created experiment. Validates that another
    /// experiment with the same name does not already exist and fails if another experiment with the same name
    /// already exists.
    pub fn create_experiment(
        &self,
        name: String,
        artifact_location: Option<String>,
        // ) -> Result<String, reqwest::Error> {
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
}

/// An error that can happend when listing `Experiment`s.
#[derive(serde::Deserialize, serde::Serialize, Debug, Clone, Copy)]
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

impl MlflowClient {
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
}
