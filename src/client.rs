/// MLFlow API Client.
#[derive(Debug)]
pub struct MlflowClient {
    /// MLFlow instance url.
    pub url: String,
    pub(crate) client: reqwest::Client,
}

impl MlflowClient {
    /// New `MlflowClient`, validating the `url`.
    pub fn new(url: String) -> Result<MlflowClient, crate::errors::SetupError> {
        match reqwest::Url::parse(&url) {
            Err(_) => Err(crate::errors::SetupError::InvalidUrl(url)),
            Ok(_) => Ok(MlflowClient {
                url,
                client: reqwest::Client::new(),
            }),
        }
    }
}
