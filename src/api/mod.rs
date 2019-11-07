mod experiments;
mod run_data;
mod runs;

use crate::errors;

/// MLFlow API Client.
#[derive(Debug)]
pub struct MLflowAPI {
    pub(crate) uri: String,
    pub(crate) client: reqwest::Client,
}

impl MLflowAPI {
    /// New `MLflowAPI`, validating the `uri`.
    pub fn new(uri: &str) -> Result<MLflowAPI, crate::errors::SetupError> {
        match reqwest::Url::parse(&uri) {
            Err(_) => Err(crate::errors::SetupError::InvalidUrl(uri.to_string())),
            Ok(_) => Ok(MLflowAPI {
                uri: uri.to_string(),
                client: reqwest::Client::new(),
            }),
        }
    }
}

#[derive(serde::Deserialize, Debug)]
#[serde(untagged)]
enum Response<T, E: errors::ErrorCode + std::fmt::Debug + serde::Serialize> {
    Error(errors::ErrorResponse<E>),
    Success(T),
}

#[derive(serde::Deserialize, Debug)]
struct EmptyResponse {}

#[inline]
pub(crate) fn send_and_return_field<
    Resp,
    ExtractedResp,
    ErrorCode: crate::errors::ErrorCode + std::fmt::Debug + serde::ser::Serialize,
    Extractor: FnOnce(Resp) -> ExtractedResp,
>(
    request: reqwest::RequestBuilder,
    extract_response: Extractor,
) -> Result<ExtractedResp, crate::errors::ClientError<ErrorCode>>
where
    for<'de> Resp: serde::de::Deserialize<'de>,
    for<'de> ErrorCode: serde::de::Deserialize<'de>,
{
    match request.send()?.json::<Response<Resp, ErrorCode>>()? {
        Response::Success(resp) => Ok(extract_response(resp)),
        Response::Error(err) => Err(err.into()),
    }
}

#[cfg(test)]
mod tests {

    use reqwest;
    use serde::{Deserialize, Serialize};
    use spectral::prelude::*;

    use super::send_and_return_field;
    use super::MLflowAPI;

    #[test]
    fn can_create_instance() {
        assert_that!(MLflowAPI::new("http://localhost:5000"))
            .is_ok()
            .map(|client| &client.uri)
            .is_equal_to("http://localhost:5000".to_string());
    }

    #[test]
    fn cant_create_instance_with_invalid_url() {
        assert_that!(MLflowAPI::new("not-a-url"))
            .is_err()
            .is_equal_to(crate::errors::SetupError::InvalidUrl(
                "not-a-url".to_string(),
            ));
    }

    #[test]
    fn can_cast_reqwest_error() {
        #[derive(Serialize, Deserialize, Debug)]
        struct CustomError;
        impl crate::errors::ErrorCode for CustomError {};

        fn test() -> Result<(), crate::errors::ClientError<CustomError>> {
            let req = reqwest::Client::new().get("http://ghghghgh");
            send_and_return_field(req, |_: ()| ())
        }
        assert_that!(test()).is_err();
    }
}
