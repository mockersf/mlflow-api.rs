use std::fs::File;
use std::io::prelude::*;

use rand::distributions::Alphanumeric;
use rand::{thread_rng, Rng};
use spectral::prelude::*;

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

    let run = mlflow.create_run(
        &id,
        Some(
            std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .expect("time went strange there")
                .as_millis() as u64,
        ),
        None,
    );
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
        .has_length(1);
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
        .has_length(1);
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
fn can_log_params() {
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

    let run = mlflow.create_run(
        &id,
        Some(
            std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .expect("time went strange there")
                .as_millis() as u64,
        ),
        None,
    );
    assert_that!(run).is_ok();
    let run_id = run.unwrap().info.run_id;

    let set = mlflow.log_param(&run_id, &key, &value1);
    assert_that!(set).is_ok();
    let run = mlflow.get_run(&run_id);
    assert_that!(run)
        .is_ok()
        .map(|run| &run.data)
        .is_some()
        .map(|data| &data.params)
        .has_length(1);
    assert_that!(run)
        .is_ok()
        .map(|run| &run.data)
        .is_some()
        .map(|data| &data.params)
        .contains(mlflow_api::Param {
            key: key.clone(),
            value: value1,
        });

    let set = mlflow.log_param(&run_id, &key, &value2);
    assert_that!(set).is_ok();
    let run = mlflow.get_run(&run_id);
    assert_that!(run)
        .is_ok()
        .map(|run| &run.data)
        .is_some()
        .map(|data| &data.params)
        .has_length(1);
    assert_that!(run)
        .is_ok()
        .map(|run| &run.data)
        .is_some()
        .map(|data| &data.params)
        .contains(mlflow_api::Param {
            key: key.clone(),
            value: value2,
        });

    mlflow.delete_experiment(&id).unwrap();
}

#[test]
fn can_log_metrics() {
    let experiment_name: String = thread_rng().sample_iter(&Alphanumeric).take(30).collect();
    let key: String = thread_rng().sample_iter(&Alphanumeric).take(30).collect();
    let ts_0 = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .expect("time went strange there")
        .as_millis() as u64;
    let value_1 = thread_rng().gen::<f32>();
    let value_2 = thread_rng().gen::<f32>();

    let mlflow = mlflow_api::MlflowClient::new(
        &std::env::var("MLFLOW_URL").unwrap_or_else(|_| "http://127.0.0.1:5000".to_string()),
    )
    .unwrap();

    let id = mlflow.create_experiment(&experiment_name, None);
    assert_that!(id).is_ok();
    let id = id.unwrap();

    let run = mlflow.create_run(&id, Some(ts_0), None);
    assert_that!(run).is_ok();
    let run_id = run.unwrap().info.run_id;

    let set = mlflow.log_metric(&run_id, &key, value_1, ts_0, None);
    assert_that!(set).is_ok();
    let run = mlflow.get_run(&run_id);
    assert_that!(run)
        .is_ok()
        .map(|run| &run.data)
        .is_some()
        .map(|data| &data.metrics)
        .has_length(1);
    assert_that!(run)
        .is_ok()
        .map(|run| &run.data)
        .is_some()
        .map(|data| &data.metrics)
        .contains(mlflow_api::Metric {
            key: key.clone(),
            value: value_1,
            timestamp: ts_0,
            step: 0,
        });

    let set = mlflow.log_metric(&run_id, &key, value_2, ts_0 + 5, None);
    assert_that!(set).is_ok();
    let run = mlflow.get_run(&run_id);
    assert_that!(run)
        .is_ok()
        .map(|run| &run.data)
        .is_some()
        .map(|data| &data.metrics)
        .has_length(1);
    assert_that!(run)
        .is_ok()
        .map(|run| &run.data)
        .is_some()
        .map(|data| &data.metrics)
        .contains(mlflow_api::Metric {
            key: key.clone(),
            value: value_2,
            timestamp: ts_0 + 5,
            step: 0,
        });

    let metrics = mlflow.get_metric_history(&run_id, &key);
    assert_that!(metrics).is_ok().has_length(2);
    assert_that!(metrics).is_ok().contains(mlflow_api::Metric {
        key: key.clone(),
        value: value_1,
        timestamp: ts_0,
        step: 0,
    });
    assert_that!(metrics).is_ok().contains(mlflow_api::Metric {
        key: key.clone(),
        value: value_2,
        timestamp: ts_0 + 5,
        step: 0,
    });

    mlflow.delete_experiment(&id).unwrap();
}

#[test]
fn can_log_batch() {
    let experiment_name: String = thread_rng().sample_iter(&Alphanumeric).take(30).collect();
    let key_tag: String = thread_rng().sample_iter(&Alphanumeric).take(30).collect();
    let value_tag: String = thread_rng().sample_iter(&Alphanumeric).take(30).collect();
    let key_param: String = thread_rng().sample_iter(&Alphanumeric).take(30).collect();
    let value_param: String = thread_rng().sample_iter(&Alphanumeric).take(30).collect();
    let key_metric: String = thread_rng().sample_iter(&Alphanumeric).take(30).collect();
    let value_metric = thread_rng().gen::<f32>();
    let ts_0 = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .expect("time went strange there")
        .as_millis() as u64;

    let mlflow = mlflow_api::MlflowClient::new(
        &std::env::var("MLFLOW_URL").unwrap_or_else(|_| "http://127.0.0.1:5000".to_string()),
    )
    .unwrap();

    let id = mlflow.create_experiment(&experiment_name, None);
    assert_that!(id).is_ok();
    let id = id.unwrap();

    let run = mlflow.create_run(&id, Some(ts_0), None);
    assert_that!(run).is_ok();
    let run_id = run.unwrap().info.run_id;

    let batch = mlflow.log_batch(&run_id, None, None, None);
    assert_that!(batch).is_ok();
    let run = mlflow.get_run(&run_id);
    assert_that!(run)
        .is_ok()
        .map(|run| &run.data)
        .is_some()
        .map(|data| &data.metrics)
        .has_length(0);
    assert_that!(run)
        .is_ok()
        .map(|run| &run.data)
        .is_some()
        .map(|data| &data.params)
        .has_length(0);
    assert_that!(run)
        .is_ok()
        .map(|run| &run.data)
        .is_some()
        .map(|data| &data.tags)
        .has_length(0);

    let batch = mlflow.log_batch(
        &run_id,
        Some(&[&mlflow_api::Metric {
            key: key_metric.clone(),
            value: value_metric,
            timestamp: ts_0 + 5,
            step: 0,
        }]),
        None,
        None,
    );
    assert_that!(batch).is_ok();
    let run = mlflow.get_run(&run_id);
    assert_that!(run)
        .is_ok()
        .map(|run| &run.data)
        .is_some()
        .map(|data| &data.metrics)
        .has_length(1);
    assert_that!(run)
        .is_ok()
        .map(|run| &run.data)
        .is_some()
        .map(|data| &data.params)
        .has_length(0);
    assert_that!(run)
        .is_ok()
        .map(|run| &run.data)
        .is_some()
        .map(|data| &data.tags)
        .has_length(0);

    let batch = mlflow.log_batch(
        &run_id,
        None,
        Some(&[&mlflow_api::Param {
            key: key_param.clone(),
            value: value_param.clone(),
        }]),
        None,
    );
    assert_that!(batch).is_ok();
    let run = mlflow.get_run(&run_id);
    assert_that!(run)
        .is_ok()
        .map(|run| &run.data)
        .is_some()
        .map(|data| &data.metrics)
        .has_length(1);
    assert_that!(run)
        .is_ok()
        .map(|run| &run.data)
        .is_some()
        .map(|data| &data.params)
        .has_length(1);
    assert_that!(run)
        .is_ok()
        .map(|run| &run.data)
        .is_some()
        .map(|data| &data.tags)
        .has_length(0);

    let batch = mlflow.log_batch(
        &run_id,
        None,
        None,
        Some(&[&mlflow_api::RunTag {
            key: key_tag.clone(),
            value: value_tag.clone(),
        }]),
    );
    assert_that!(batch).is_ok();
    let run = mlflow.get_run(&run_id);
    assert_that!(run)
        .is_ok()
        .map(|run| &run.data)
        .is_some()
        .map(|data| &data.metrics)
        .has_length(1);
    assert_that!(run)
        .is_ok()
        .map(|run| &run.data)
        .is_some()
        .map(|data| &data.params)
        .has_length(1);
    assert_that!(run)
        .is_ok()
        .map(|run| &run.data)
        .is_some()
        .map(|data| &data.tags)
        .has_length(1);

    mlflow.delete_experiment(&id).unwrap();
}

#[test]
fn can_list_artifacts() {
    let experiment_name: String = thread_rng().sample_iter(&Alphanumeric).take(30).collect();
    let file_name: String = thread_rng().sample_iter(&Alphanumeric).take(30).collect();
    let content: String = thread_rng().sample_iter(&Alphanumeric).take(30).collect();

    let mlflow = mlflow_api::MlflowClient::new(
        &std::env::var("MLFLOW_URL").unwrap_or_else(|_| "http://127.0.0.1:5000".to_string()),
    )
    .unwrap();
    let storage = std::env::var("MLFLOW_PATH").unwrap_or_else(|_| "/tmp/mlruns".to_string());

    let id = mlflow.create_experiment(&experiment_name, None);
    assert_that!(id).is_ok();
    let id = id.unwrap();

    let run = mlflow.create_run(
        &id,
        Some(
            std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .expect("time went strange there")
                .as_millis() as u64,
        ),
        None,
    );
    assert_that!(run).is_ok();
    let run_id = run.unwrap().info.run_id;

    let artifacts = mlflow.list_artifacts(&run_id, None);
    assert_that!(artifacts)
        .is_ok()
        .map(|artifacts| &artifacts.1)
        .has_length(0);
    let root_uri = artifacts.unwrap().0;

    if let Some(github_dir) = std::env::var("GITHUB_WORKSPACE") {
        let mut file =
            File::create(format!("{}/{}", github_dir, file_name)).expect("error creating file");
        file.write_all(content.as_bytes())
            .expect("error writing file");
        std::process::Command::new("docker")
            .arg("cp")
            .arg(file_name.clone())
            .arg(format!(
                "mlflow-full-integration-tests:/mlflow/{}/{}",
                root_uri, file_name
            ))
            .output()
            .expect("error copying file to docker");
    } else {
        let mut file = File::create(format!("{}/{}/{}", storage, root_uri, file_name))
            .expect("error creating file");
        file.write_all(content.as_bytes())
            .expect("error writing file");
    }

    let artifacts = mlflow.list_artifacts(&run_id, None);
    assert_that!(artifacts)
        .is_ok()
        .map(|artifacts| &artifacts.1)
        .has_length(1);
    assert_that!(artifacts)
        .is_ok()
        .map(|artifacts| &artifacts.1)
        .contains(mlflow_api::FileInfo {
            path: file_name,
            is_dir: false,
            file_size: Some(30),
        });

    mlflow.delete_experiment(&id).unwrap();
}
