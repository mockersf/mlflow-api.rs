use rand::distributions::Alphanumeric;
use rand::{thread_rng, Rng};
use spectral::prelude::*;

#[test]
fn can_create_experiment() {
    let experiment_name: String = thread_rng().sample_iter(&Alphanumeric).take(30).collect();

    let mlflow = mlflow_api::MLflowAPI::new(
        &std::env::var("MLFLOW_TRACKING_URL")
            .unwrap_or_else(|_| "http://127.0.0.1:5000".to_string()),
    )
    .unwrap();

    let id = mlflow.create_experiment(&experiment_name, None);
    assert_that!(id).is_ok();

    mlflow.delete_experiment(&id.unwrap()).unwrap();
}

#[cfg(feature = "integration-tests")]
#[test]
fn cant_create_2_experiments_with_same_name() {
    let experiment_name: String = thread_rng().sample_iter(&Alphanumeric).take(30).collect();

    let mlflow = mlflow_api::MLflowAPI::new(
        &std::env::var("MLFLOW_TRACKING_URL")
            .unwrap_or_else(|_| "http://127.0.0.1:5000".to_string()),
    )
    .unwrap();

    let id = mlflow.create_experiment(&experiment_name, None);
    assert_that!(id).is_ok();

    let id2 = mlflow.create_experiment(&experiment_name, None);
    assert_that!(id2)
        .is_err()
        .is_equal_to(mlflow_api::errors::ClientError::ApiError {
            error_code: mlflow_api::errors::CreateExperimentErrorCode::ResourceAlreadyExists,
            message: format!("Experiment '{}' already exists.", experiment_name),
        });

    mlflow.delete_experiment(&id.unwrap()).unwrap();
}

#[test]
fn can_get_experiment() {
    let experiment_name: String = thread_rng().sample_iter(&Alphanumeric).take(30).collect();

    let mlflow = mlflow_api::MLflowAPI::new(
        &std::env::var("MLFLOW_TRACKING_URL")
            .unwrap_or_else(|_| "http://127.0.0.1:5000".to_string()),
    )
    .unwrap();

    let id = mlflow.create_experiment(&experiment_name, None);
    assert_that!(id).is_ok();
    let id = id.unwrap();

    let experiment = mlflow.get_experiment(&id);
    assert_that!(experiment)
        .is_ok()
        .map(|experiment| &experiment.experiment_id)
        .is_equal_to(&id);

    mlflow.delete_experiment(&id).unwrap();
}

#[test]
#[cfg(feature = "integration-tests")]
fn cant_get_unknown_experiment() {
    let experiment_id: String = thread_rng().sample_iter(&Alphanumeric).take(30).collect();

    let mlflow = mlflow_api::MLflowAPI::new(
        &std::env::var("MLFLOW_TRACKING_URL")
            .unwrap_or_else(|_| "http://127.0.0.1:5000".to_string()),
    )
    .unwrap();

    let experiment = mlflow.get_experiment(&experiment_id);
    assert_that!(experiment)
        .is_err()
        .is_equal_to(mlflow_api::errors::ClientError::ApiError {
            error_code: mlflow_api::errors::GetExperimentErrorCode::ResourceDoesNotExist,
            message: format!("Could not find experiment with ID {}", experiment_id),
        });
}

#[test]
fn can_get_experiment_by_name() {
    let experiment_name: String = thread_rng().sample_iter(&Alphanumeric).take(30).collect();

    let mlflow = mlflow_api::MLflowAPI::new(
        &std::env::var("MLFLOW_TRACKING_URL")
            .unwrap_or_else(|_| "http://127.0.0.1:5000".to_string()),
    )
    .unwrap();

    let id = mlflow.create_experiment(&experiment_name, None);
    assert_that!(id).is_ok();
    let id = id.unwrap();

    let experiment = mlflow.get_experiment_by_name(&experiment_name);
    assert_that!(experiment)
        .is_ok()
        .map(|experiment| &experiment.experiment_id)
        .is_equal_to(&id);

    mlflow.delete_experiment(&id).unwrap();
}

#[test]
fn can_update_experiment() {
    let experiment_name: String = thread_rng().sample_iter(&Alphanumeric).take(30).collect();
    let new_experiment_name: String = thread_rng().sample_iter(&Alphanumeric).take(30).collect();

    let mlflow = mlflow_api::MLflowAPI::new(
        &std::env::var("MLFLOW_TRACKING_URL")
            .unwrap_or_else(|_| "http://127.0.0.1:5000".to_string()),
    )
    .unwrap();

    let id = mlflow.create_experiment(&experiment_name, None);
    assert_that!(id).is_ok();
    let id = id.unwrap();

    let experiment = mlflow.get_experiment(&id);
    assert_that!(experiment)
        .is_ok()
        .map(|experiment| &experiment.name)
        .is_equal_to(&experiment_name);

    let rename = mlflow.update_experiment(&id, &new_experiment_name);
    assert_that!(rename).is_ok();

    let experiment = mlflow.get_experiment(&id);
    assert_that!(experiment)
        .is_ok()
        .map(|experiment| &experiment.name)
        .is_equal_to(&new_experiment_name);

    mlflow.delete_experiment(&id).unwrap();
}

#[test]
fn can_delete_and_restore_experiment() {
    let experiment_name: String = thread_rng().sample_iter(&Alphanumeric).take(30).collect();

    let mlflow = mlflow_api::MLflowAPI::new(
        &std::env::var("MLFLOW_TRACKING_URL")
            .unwrap_or_else(|_| "http://127.0.0.1:5000".to_string()),
    )
    .unwrap();

    let id = mlflow.create_experiment(&experiment_name, None);
    assert_that!(id).is_ok();
    let id = id.unwrap();

    let experiment = mlflow.get_experiment(&id);
    assert_that!(experiment)
        .is_ok()
        .map(|experiment| &experiment.lifecycle_stage)
        .is_equal_to(&mlflow_api::LifecycleStage::Active);

    let delete = mlflow.delete_experiment(&id);
    assert_that!(delete).is_ok();

    let experiment = mlflow.get_experiment(&id);
    assert_that!(experiment)
        .is_ok()
        .map(|experiment| &experiment.lifecycle_stage)
        .is_equal_to(&mlflow_api::LifecycleStage::Deleted);

    let restore = mlflow.restore_experiment(&id);
    assert_that!(restore).is_ok();

    let experiment = mlflow.get_experiment(&id);
    assert_that!(experiment)
        .is_ok()
        .map(|experiment| &experiment.lifecycle_stage)
        .is_equal_to(&mlflow_api::LifecycleStage::Active);

    mlflow.delete_experiment(&id).unwrap();
}

#[test]
fn can_tag_experiment() {
    let experiment_name: String = thread_rng().sample_iter(&Alphanumeric).take(30).collect();
    let tag_key: String = thread_rng().sample_iter(&Alphanumeric).take(30).collect();
    let tag_value: String = thread_rng().sample_iter(&Alphanumeric).take(30).collect();

    let mlflow = mlflow_api::MLflowAPI::new(
        &std::env::var("MLFLOW_TRACKING_URL")
            .unwrap_or_else(|_| "http://127.0.0.1:5000".to_string()),
    )
    .unwrap();

    let id = mlflow.create_experiment(&experiment_name, None);
    assert_that!(id).is_ok();
    let id = id.unwrap();

    let experiment = mlflow.get_experiment(&id);
    assert_that!(experiment)
        .is_ok()
        .map(|experiment| &experiment.tags)
        .is_none();

    let tag = mlflow.set_experiment_tag(&id, &tag_key, &tag_value);
    assert_that!(tag).is_ok();

    let experiment = mlflow.get_experiment(&id);
    assert_that!(experiment)
        .is_ok()
        .map(|experiment| &experiment.tags)
        .is_some()
        .has_length(1);
    let tag = experiment.unwrap().tags.unwrap()[0].clone();
    assert_that!(tag.key).is_equal_to(tag_key);
    assert_that!(tag.value).is_equal_to(tag_value);

    mlflow.delete_experiment(&id).unwrap();
}

#[test]
fn can_list_experiments() {
    let mlflow = mlflow_api::MLflowAPI::new(
        &std::env::var("MLFLOW_TRACKING_URL")
            .unwrap_or_else(|_| "http://127.0.0.1:5000".to_string()),
    )
    .unwrap();

    let list = mlflow.list_experiments(None);
    assert_that!(list).is_ok();
    let base_active = list.unwrap().len();

    let list = mlflow.list_experiments(Some(mlflow_api::ViewType::DeletedOnly));
    assert_that!(list).is_ok();
    let base_deleted = list.unwrap().len();

    let experiment_name: String = thread_rng().sample_iter(&Alphanumeric).take(30).collect();
    let id = mlflow.create_experiment(&experiment_name, None);
    assert_that!(id).is_ok();
    let id_1 = id.unwrap();

    let experiment_name: String = thread_rng().sample_iter(&Alphanumeric).take(30).collect();
    let id = mlflow.create_experiment(&experiment_name, None);
    assert_that!(id).is_ok();
    let id_2 = id.unwrap();
    let delete = mlflow.delete_experiment(&id_2);
    assert_that!(delete).is_ok();

    let list = mlflow.list_experiments(None);
    assert_that!(list).is_ok().has_length(base_active + 1);
    assert_that!(list
        .unwrap()
        .iter()
        .map(|experiment| &experiment.experiment_id))
    .contains_all_of(&vec!["0".to_string(), id_1.clone()].iter());

    let list = mlflow.list_experiments(Some(mlflow_api::ViewType::ActiveOnly));
    assert_that!(list).is_ok().has_length(base_active + 1);
    assert_that!(list
        .unwrap()
        .iter()
        .map(|experiment| &experiment.experiment_id))
    .contains_all_of(&vec!["0".to_string(), id_1.clone()].iter());

    let list = mlflow.list_experiments(Some(mlflow_api::ViewType::DeletedOnly));
    assert_that!(list).is_ok().has_length(base_deleted + 1);
    assert_that!(list
        .unwrap()
        .iter()
        .map(|experiment| &experiment.experiment_id))
    .contains(&id_2);

    let list = mlflow.list_experiments(Some(mlflow_api::ViewType::All));
    assert_that!(list)
        .is_ok()
        .has_length(base_active + base_deleted + 2);
    assert_that!(list
        .unwrap()
        .iter()
        .map(|experiment| &experiment.experiment_id))
    .contains_all_of(&vec!["0".to_string(), id_1.clone(), id_2].iter());

    mlflow.delete_experiment(&id_1).unwrap();
}
