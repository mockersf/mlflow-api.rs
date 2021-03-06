//! Errors

/// An error at the creation of the client
#[derive(Debug, PartialEq)]
pub enum SetupError {
    /// URL is invalid.
    InvalidUrl(String),
}

impl std::error::Error for SetupError {}
impl std::fmt::Display for SetupError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            SetupError::InvalidUrl(url) => write!(f, "Invalid URL: '{}'", url),
        }
    }
}

#[derive(serde::Deserialize, Debug)]
pub(crate) struct ErrorResponse<E: ErrorCode + std::fmt::Debug + serde::Serialize> {
    /// The error code.
    pub error_code: E,
    /// The error message.
    pub message: String,
}

/// An error that append when calling the API.
#[derive(Debug)]
pub enum ClientError<E: ErrorCode> {
    /// An API error.
    ApiError {
        /// The error code.
        error_code: E,
        /// The error message.
        message: String,
    },
    /// A request error.
    QueryError(reqwest::Error),
}

#[cfg(feature = "integration-tests")]
impl<E: ErrorCode + PartialEq> PartialEq for ClientError<E> {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (ClientError::QueryError(_), _) => unimplemented!(),
            (_, ClientError::QueryError(_)) => unimplemented!(),
            (
                ClientError::ApiError {
                    error_code: error_code_1,
                    message: message_1,
                },
                ClientError::ApiError {
                    error_code: error_code_2,
                    message: message_2,
                },
            ) => error_code_1 == error_code_2 && message_1 == message_2,
        }
    }
}

impl<'d, E: ErrorCode + std::fmt::Debug + serde::Serialize> std::error::Error for ClientError<E> {}

impl<'d, E: ErrorCode + serde::Serialize> std::fmt::Display for ClientError<E> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ClientError::QueryError(error) => error.fmt(f),
            ClientError::ApiError {
                error_code,
                message,
            } => write!(
                f,
                "{}: {}",
                serde_json::to_string(error_code).expect("error serializing error_code"),
                message
            ),
        }
    }
}

impl<E: ErrorCode> From<reqwest::Error> for ClientError<E> {
    fn from(error: reqwest::Error) -> ClientError<E> {
        ClientError::QueryError(error)
    }
}

impl<E: ErrorCode + std::fmt::Debug + serde::Serialize> From<ErrorResponse<E>> for ClientError<E> {
    fn from(error: ErrorResponse<E>) -> ClientError<E> {
        ClientError::ApiError {
            error_code: error.error_code,
            message: error.message,
        }
    }
}

#[doc(hidden)]
pub trait ErrorCode {}

/// An error that can happen when getting an `Experiment`.
#[derive(serde::Deserialize, serde::Serialize, Debug, Copy, Clone, PartialEq)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum GetExperimentErrorCode {
    /// An experiment with the requested ID could not be found.
    ResourceDoesNotExist,
    /// An internal error, more information in the associated message.
    InternalError,
    /// A parameter has an invalid value
    InvalidParameterValue,
    /// Unknown error.
    #[serde(other)]
    UnknownError,
}
impl ErrorCode for GetExperimentErrorCode {}

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
impl ErrorCode for CreateExperimentErrorCode {}

/// An error that can happen when listing `Experiment`s.
#[derive(serde::Deserialize, serde::Serialize, Debug, Clone, Copy, PartialEq)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum ListExperimentsErrorCode {
    /// Unknown error.
    #[serde(other)]
    UnknownError,
}
impl ErrorCode for ListExperimentsErrorCode {}
