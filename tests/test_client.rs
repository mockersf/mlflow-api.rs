use spectral::prelude::*;

#[test]
fn can_create_with_valid_url() {
    let mlflow = mlflow_api::MlflowClient::new(
        &std::env::var("MLFLOW_URL").unwrap_or_else(|_| "http://127.0.0.1:5000".to_string()),
    );

    assert_that!(mlflow).is_ok();
}

#[test]
fn cant_create_with_invalid_url() {
    let mlflow = mlflow_api::MlflowClient::new("not-a-url");

    assert_that!(mlflow).is_err();
}
