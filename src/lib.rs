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

#[derive(serde::Deserialize, Debug)]
#[serde(untagged)]
enum Response<T, E: errors::ErrorCode + std::fmt::Debug + serde::Serialize> {
    Error(errors::ErrorResponse<E>),
    Success(T),
}
