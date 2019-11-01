use rand::distributions::Alphanumeric;
use rand::{thread_rng, Rng};
use spectral::prelude::*;

#[test]
fn can_create_run() {
    let experiment_name: String = thread_rng().sample_iter(&Alphanumeric).take(30).collect();

    let mlflow = mlflow_api::MlflowClient::new(
        std::env::var("MLFLOW_URL").unwrap_or_else(|_| "http://127.0.0.1:5000".to_string()),
    )
    .unwrap();

    let id = mlflow.create_experiment(&experiment_name, None);
    assert_that!(id).is_ok();
    let id = id.unwrap();

    let run = mlflow.create_run(&id, Some(514425600000), None);
    assert_that!(run).is_ok();
    let run = run.unwrap();

    assert_that!(run)
        .map(|run| &run.info.experiment_id)
        .is_equal_to(&id);
    assert_that!(run)
        .map(|run| &run.info.status)
        .is_equal_to(mlflow_api::RunStatus::Running);
    assert_that!(run)
        .map(|run| &run.info.lifecycle_stage)
        .is_equal_to(mlflow_api::LifecycleStage::Active);

    mlflow.delete_experiment(&id).unwrap();
}

#[test]
fn can_get_run() {
    let experiment_name: String = thread_rng().sample_iter(&Alphanumeric).take(30).collect();

    let mlflow = mlflow_api::MlflowClient::new(
        std::env::var("MLFLOW_URL").unwrap_or_else(|_| "http://127.0.0.1:5000".to_string()),
    )
    .unwrap();

    let id = mlflow.create_experiment(&experiment_name, None);
    assert_that!(id).is_ok();
    let id = id.unwrap();

    let create = mlflow.create_run(&id, Some(514425600000), None);
    assert_that!(create).is_ok();

    let run = mlflow.get_run(&create.unwrap().info.run_id);
    assert_that!(run).is_ok();
    let run = run.unwrap();

    assert_that!(run)
        .map(|run| &run.info.experiment_id)
        .is_equal_to(&id);
    assert_that!(run)
        .map(|run| &run.info.status)
        .is_equal_to(mlflow_api::RunStatus::Running);
    assert_that!(run)
        .map(|run| &run.info.lifecycle_stage)
        .is_equal_to(mlflow_api::LifecycleStage::Active);

    mlflow.delete_experiment(&id).unwrap();
}

#[test]
fn can_delete_and_restore_run() {
    let experiment_name: String = thread_rng().sample_iter(&Alphanumeric).take(30).collect();

    let mlflow = mlflow_api::MlflowClient::new(
        std::env::var("MLFLOW_URL").unwrap_or_else(|_| "http://127.0.0.1:5000".to_string()),
    )
    .unwrap();

    let id = mlflow.create_experiment(&experiment_name, None);
    assert_that!(id).is_ok();
    let id = id.unwrap();

    let run = mlflow.create_run(&id, Some(514425600000), None);
    assert_that!(run).is_ok();
    let run_id = run.unwrap().info.run_id;

    let delete = mlflow.delete_run(&run_id);
    assert_that!(delete).is_ok();
    let get = mlflow.get_run(&run_id);
    assert_that!(get)
        .is_ok()
        .map(|run| &run.info.lifecycle_stage)
        .is_equal_to(mlflow_api::LifecycleStage::Deleted);

    let restore = mlflow.restore_run(&run_id);
    assert_that!(restore).is_ok();
    let get = mlflow.get_run(&run_id);
    assert_that!(get)
        .is_ok()
        .map(|run| &run.info.lifecycle_stage)
        .is_equal_to(mlflow_api::LifecycleStage::Active);

    mlflow.delete_experiment(&id).unwrap();
}

#[test]
fn can_update_run() {
    let experiment_name: String = thread_rng().sample_iter(&Alphanumeric).take(30).collect();

    let mlflow = mlflow_api::MlflowClient::new(
        std::env::var("MLFLOW_URL").unwrap_or_else(|_| "http://127.0.0.1:5000".to_string()),
    )
    .unwrap();

    let id = mlflow.create_experiment(&experiment_name, None);
    assert_that!(id).is_ok();
    let id = id.unwrap();

    let run = mlflow.create_run(&id, Some(514425600000), None);
    assert_that!(run).is_ok();
    let run_id = run.unwrap().info.run_id;

    // Not changing the status
    let updated = mlflow.update_run(&run_id, mlflow_api::RunStatus::Running, None);
    assert_that!(updated)
        .is_ok()
        .map(|run_info| &run_info.status)
        .is_equal_to(mlflow_api::RunStatus::Running);

    // Changing to scheduled
    let updated = mlflow.update_run(&run_id, mlflow_api::RunStatus::Scheduled, None);
    assert_that!(updated)
        .is_ok()
        .map(|run_info| &run_info.status)
        .is_equal_to(mlflow_api::RunStatus::Scheduled);

    // Changing to finished with an end time
    let updated = mlflow.update_run(&run_id, mlflow_api::RunStatus::Finished, Some(514425600001));
    assert_that!(updated).is_ok();
    let updated = updated.unwrap();
    assert_that!(updated)
        .map(|run_info| &run_info.status)
        .is_equal_to(mlflow_api::RunStatus::Finished);
    assert_that!(updated)
        .map(|run_info| &run_info.end_time)
        .is_equal_to(Some(514425600001));

    mlflow.delete_experiment(&id).unwrap();
}
