use rand::distributions::Alphanumeric;
use rand::{thread_rng, Rng};
use spectral::prelude::*;

#[test]
fn can_create_run() {
    let experiment_name: String = thread_rng().sample_iter(&Alphanumeric).take(30).collect();

    let mlflow = mlflow_api::MlflowClient::new(
        &std::env::var("MLFLOW_URL").unwrap_or_else(|_| "http://127.0.0.1:5000".to_string()),
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
        &std::env::var("MLFLOW_URL").unwrap_or_else(|_| "http://127.0.0.1:5000".to_string()),
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
        &std::env::var("MLFLOW_URL").unwrap_or_else(|_| "http://127.0.0.1:5000".to_string()),
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
        &std::env::var("MLFLOW_URL").unwrap_or_else(|_| "http://127.0.0.1:5000".to_string()),
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

#[test]
fn can_manage_run_tags() {
    let experiment_name: String = thread_rng().sample_iter(&Alphanumeric).take(30).collect();
    let key: String = thread_rng().sample_iter(&Alphanumeric).take(30).collect();
    let value1: String = thread_rng().sample_iter(&Alphanumeric).take(30).collect();
    let value2: String = thread_rng().sample_iter(&Alphanumeric).take(30).collect();

    let mlflow = mlflow_api::MlflowClient::new(
        &std::env::var("MLFLOW_URL").unwrap_or_else(|_| "http://127.0.0.1:5000".to_string()),
    )
    .unwrap();

    let id = mlflow.create_experiment(&experiment_name, None);
    assert_that!(id).is_ok();
    let id = id.unwrap();

    let run = mlflow.create_run(&id, Some(514425600000), None);
    assert_that!(run).is_ok();
    let run_id = run.unwrap().info.run_id;

    let set = mlflow.set_run_tag(&run_id, &key, &value1);
    assert_that!(set).is_ok();
    let run = mlflow.get_run(&run_id);
    assert_that!(run)
        .is_ok()
        .map(|run| &run.data)
        .is_some()
        .map(|data| &data.tags)
        .contains(mlflow_api::RunTag {
            key: key.clone(),
            value: value1,
        });

    let set = mlflow.set_run_tag(&run_id, &key, &value2);
    assert_that!(set).is_ok();
    let run = mlflow.get_run(&run_id);
    assert_that!(run)
        .is_ok()
        .map(|run| &run.data)
        .is_some()
        .map(|data| &data.tags)
        .contains(mlflow_api::RunTag {
            key: key.clone(),
            value: value2,
        });

    let set = mlflow.delete_run_tag(&run_id, &key);
    assert_that!(set).is_ok();
    let run = mlflow.get_run(&run_id);
    assert_that!(run)
        .is_ok()
        .map(|run| &run.data)
        .is_some()
        .map(|data| &data.tags)
        .has_length(0);

    mlflow.delete_experiment(&id).unwrap();
}

#[test]
fn can_search_for_runs() {
    let experiment_name_1: String = thread_rng().sample_iter(&Alphanumeric).take(30).collect();
    let experiment_name_2: String = thread_rng().sample_iter(&Alphanumeric).take(30).collect();

    let mlflow = mlflow_api::MlflowClient::new(
        &std::env::var("MLFLOW_URL").unwrap_or_else(|_| "http://127.0.0.1:5000".to_string()),
    )
    .unwrap();

    let id = mlflow.create_experiment(&experiment_name_1, None);
    assert_that!(id).is_ok();
    let id_1 = id.unwrap();

    let id = mlflow.create_experiment(&experiment_name_2, None);
    assert_that!(id).is_ok();
    let id_2 = id.unwrap();

    let run = mlflow.create_run(
        &id_1,
        Some(
            std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .expect("time went strange there")
                .as_millis() as u64,
        ),
        None,
    );
    assert_that!(run).is_ok();

    let run = mlflow.create_run(
        &id_2,
        Some(
            std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .expect("time went strange there")
                .as_millis() as u64,
        ),
        None,
    );
    assert_that!(run).is_ok();

    let run = mlflow.create_run(
        &id_2,
        Some(
            std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .expect("time went strange there")
                .as_millis() as u64,
        ),
        None,
    );
    assert_that!(run).is_ok();

    let runs = mlflow.search_runs(&[&id_1], None, None, None, None, None);
    assert_that!(runs).is_ok().map(|runs| &runs.0).has_length(1);

    let runs = mlflow.search_runs(&[&id_2], None, None, None, None, None);
    assert_that!(runs).is_ok().map(|runs| &runs.0).has_length(2);

    let runs = mlflow.search_runs(&[&id_1, &id_2], None, None, None, None, None);
    assert_that!(runs).is_ok().map(|runs| &runs.0).has_length(3);

    mlflow.delete_experiment(&id_1).unwrap();
    mlflow.delete_experiment(&id_2).unwrap();
}
