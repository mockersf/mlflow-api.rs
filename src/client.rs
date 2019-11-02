/// MLFlow API Client.
#[derive(Debug)]
pub struct MlflowClient {
    pub(crate) url: String,
    pub(crate) client: reqwest::Client,
}

impl MlflowClient {
    /// New `MlflowClient`, validating the `url`.
    pub fn new(url: &str) -> Result<MlflowClient, crate::errors::SetupError> {
        match reqwest::Url::parse(&url) {
            Err(_) => Err(crate::errors::SetupError::InvalidUrl(url.to_string())),
            Ok(_) => Ok(MlflowClient {
                url: url.to_string(),
                client: reqwest::Client::new(),
            }),
        }
    }
}

#[cfg(test)]
mod tests {
    use spectral::prelude::*;

    use super::MlflowClient;

    #[test]
    fn can_create_instance() {
        assert_that!(MlflowClient::new("http://localhost:5000"))
            .is_ok()
            .map(|client| &client.url)
            .is_equal_to("http://localhost:5000".to_string());
    }

    #[test]
    fn cant_create_instance_with_invalid_url() {
        assert_that!(MlflowClient::new("not-a-url"))
            .is_err()
            .is_equal_to(crate::errors::SetupError::InvalidUrl(
                "not-a-url".to_string(),
            ));
    }
}
