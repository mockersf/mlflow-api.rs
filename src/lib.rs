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

mod api;
pub use api::MLflowAPI;
mod structures;
pub use structures::*;
pub mod errors;

mod client;
pub use client::MLflowClient;
