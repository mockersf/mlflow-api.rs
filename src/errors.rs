//! Errors

/// An error at the creation of the client
#[derive(Debug)]
pub enum SetupError {
    /// URL is invalid.
    InvalidUrl(String),
}
// pub struct InvalidUrl(pub String);

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
