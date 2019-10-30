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

/// MLFlow API Client.
#[derive(Debug)]
pub struct MlflowClient {
    /// MLFlow instance url.
    pub url: String,
}

#[derive(serde::Serialize, Debug)]
struct CreateExperimentsQuery {
    name: String,
    artifact_location: Option<String>,
}

#[derive(serde::Deserialize, Debug)]
struct CreateExperimentsResponse {
    experiment_id: String,
}

#[derive(serde::Deserialize, Debug)]
struct ListExperimentsResponse {
    experiments: Vec<Experiment>,
}

#[derive(serde::Deserialize, Debug)]
struct ErrorResponse {
    error_code: ErrorCode,
    message: String,
}

#[derive(serde::Deserialize, Debug)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
enum ErrorCode {
    ResourceAlreadyExists,
}

#[derive(serde::Deserialize, Debug)]
#[serde(untagged)]
enum Response<T> {
    Success(T),
    Error(ErrorResponse),
}

impl MlflowClient {
    /// Create an experiment with a name. Returns the ID of the newly created experiment. Validates that another
    /// experiment with the same name does not already exist and fails if another experiment with the same name
    /// already exists.
    pub fn create_experiment(
        &self,
        name: String,
        artifact_location: Option<String>,
    ) -> Result<String, reqwest::Error> {
        let client = reqwest::Client::new();
        let mut req = client.post(&format!("{}/api/2.0/mlflow/experiments/create", self.url));
        req = req.json(&CreateExperimentsQuery {
            name,
            artifact_location,
        });
        match req.send()?.json::<Response<CreateExperimentsResponse>>()? {
            Response::Success(resp) => Ok(resp.experiment_id),
            Response::Error(err) => {
                eprintln!("error: {:?}", err);
                unimplemented!()
            }
        }
    }

    /// Get a list of all experiments.
    pub fn list_experiments(
        &self,
        view_type: Option<ViewType>,
    ) -> Result<Vec<Experiment>, reqwest::Error> {
        let client = reqwest::Client::new();
        let mut req = client.get(&format!("{}/api/2.0/mlflow/experiments/list", self.url));
        if let Some(view_type) = view_type {
            req = req.query(&[("view_type", view_type)]);
        }

        match req.send()?.json::<Response<ListExperimentsResponse>>()? {
            Response::Success(resp) => Ok(resp.experiments),
            Response::Error(err) => {
                eprintln!("error: {:?}", err);
                unimplemented!()
            }
        }
    }
}
