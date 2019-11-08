use rand::distributions::Alphanumeric;
use rand::{thread_rng, Rng};
use spectral::prelude::*;

#[test]
fn can_create_with_valid_url() {
    let mlflow = mlflow_api::MLflowClient::new_with_tracking_uri(
        &std::env::var("MLFLOW_TRACKING_URL")
            .unwrap_or_else(|_| "http://127.0.0.1:5000".to_string()),
    );

    assert_that!(mlflow).is_ok();
}

#[test]
fn can_run_basic_flow() {
    let run_name: String = thread_rng().sample_iter(&Alphanumeric).take(30).collect();
    let metric_value_1 = thread_rng().gen::<f32>();
    let metric_value_2 = thread_rng().gen::<f32>();

    let mut mlflow = mlflow_api::MLflowClient::new_with_tracking_uri(
        &std::env::var("MLFLOW_TRACKING_URL")
            .unwrap_or_else(|_| "http://127.0.0.1:5000".to_string()),
    )
    .unwrap();

    let start = mlflow.start_run(&run_name);
    assert_that!(start).is_ok();

    let log = mlflow.log_metric("metric1", metric_value_1);
    assert_that!(log).is_ok();

    let log = mlflow.log_metric("metric2", metric_value_2);
    assert_that!(log).is_ok();

    let finish = mlflow.end_run();
    assert_that!(finish).is_ok();

    let run = mlflow.active_run();
    assert_that!(run).is_ok();
    let run = run.unwrap();
    assert_that!(run)
        .map(|run| &run.info.status)
        .is_equal_to(mlflow_api::RunStatus::Finished);
    assert_that!(run).map(|run| &run.info.end_time).is_some();
    assert_that!(run).map(|run| &run.data).is_some();
    let run_data = run.data.unwrap();
    assert_that!(run_data)
        .map(|data| &data.metrics)
        .has_length(2);
    assert_that!(run_data).map(|data| &data.tags).has_length(2);
}

#[test]
fn can_create_everything_on_the_fly() {
    let experiment_name: String = thread_rng().sample_iter(&Alphanumeric).take(30).collect();
    let metric_value_1 = thread_rng().gen::<f32>();

    std::env::set_var("MLFLOW_EXPERIMENT_NAME", &experiment_name);

    let mut mlflow = mlflow_api::MLflowClient::new_with_tracking_uri(
        &std::env::var("MLFLOW_TRACKING_URL")
            .unwrap_or_else(|_| "http://127.0.0.1:5000".to_string()),
    )
    .unwrap();

    let log = mlflow.log_metric("metric1", metric_value_1);

    std::env::remove_var("MLFLOW_EXPERIMENT_NAME");

    assert_that!(log).is_ok();

    let update = mlflow.update_run_status(mlflow_api::RunStatus::Scheduled);
    assert_that!(update).is_ok();

    let run = mlflow.active_run();
    assert_that!(run).is_ok();
    let run = run.unwrap();
    assert_that!(run)
        .map(|run| &run.info.status)
        .is_equal_to(mlflow_api::RunStatus::Scheduled);
    assert_that!(run).map(|run| &run.data).is_some();
    let run_data = run.data.unwrap();
    assert_that!(run_data)
        .map(|data| &data.metrics)
        .has_length(1);
    assert_that!(run_data).map(|data| &data.tags).has_length(1);

    let experiment = mlflow.api.get_experiment(&run.info.experiment_id);
    assert_that!(experiment)
        .is_ok()
        .map(|experiment| &experiment.name)
        .is_equal_to(experiment_name);
}
