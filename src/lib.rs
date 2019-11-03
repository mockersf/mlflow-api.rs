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
mod experiments;
mod run_data;
mod runs;

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

    #[test]
    fn zut() {
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
